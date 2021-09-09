use anyhow::{bail, Context, Result};
use candid::{
    check_prog,
    parser::value::IDLValue,
    types::{Function, Type},
    CandidType, Decode, Deserialize, IDLArgs, IDLProg, TypeEnv,
};
use clap::{crate_authors, crate_version, AppSettings, Clap};
use ic_agent::{
    agent::{self, signed::SignedUpdate, Replied},
    agent::{
        agent_error::HttpErrorPayload,
        signed::{SignedQuery, SignedRequestStatus},
    },
    export::Principal,
    identity::BasicIdentity,
    Agent, AgentError, Identity,
};
use ic_utils::interfaces::management_canister::{
    builders::{CanisterInstall, CanisterSettings},
    MgmtMethod,
};
use ring::signature::Ed25519KeyPair;
use std::{
    collections::VecDeque, convert::TryFrom, io::BufRead, path::PathBuf, process::exit,
    str::FromStr,
};

use std::fs;
use hex::ToHex;
const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";


fn print_idl_blob(
    blob: &[u8],
    output_type: &ArgType,
    method_type: &Option<(TypeEnv, Function)>,
) -> Result<()> {
    let hex_string = hex::encode(blob);
    match output_type {
        ArgType::Raw => {
            println!("{}", hex_string);
        }
        ArgType::Idl => {
            let result = match method_type {
                None => candid::IDLArgs::from_bytes(blob),
                Some((env, func)) => candid::IDLArgs::from_bytes_with_types(blob, &env, &func.rets),
            };
            println!(
                "{}",
                result.with_context(|| format!("Failed to deserialize blob 0x{}", hex_string))?
            );
        }
    }
    Ok(())
}

async fn fetch_root_key_from_non_ic(agent: &Agent, replica: &str) -> Result<()> {
    let normalized_replica = replica.strip_suffix("/").unwrap_or(replica);
    if normalized_replica != DEFAULT_IC_GATEWAY {
        agent
            .fetch_root_key()
            .await
            .context("Failed to fetch root key from replica")?;
    }
    Ok(())
}

/// Parse IDL file into TypeEnv. This is a best effort function: it will succeed if
/// the IDL file can be parsed and type checked in Rust parser, and has an
/// actor in the IDL file. If anything fails, it returns None.
pub fn get_candid_type(
    idl_path: &std::path::Path,
    method_name: &str,
) -> Result<Option<(TypeEnv, Function)>> {
    let (env, ty) = check_candid_file(idl_path).with_context(|| {
        format!(
            "Failed when checking candid file: {}",
            idl_path.to_string_lossy()
        )
    })?;
    match ty {
        None => Ok(None),
        Some(actor) => {
            let method = env
                .get_method(&actor, method_name)
                .with_context(|| format!("Failed to get method: {}", method_name))?
                .clone();
            Ok(Some((env, method)))
        }
    }
}

pub fn check_candid_file(idl_path: &std::path::Path) -> Result<(TypeEnv, Option<Type>)> {
    let idl_file = std::fs::read_to_string(idl_path)
        .with_context(|| format!("Failed to read Candid file: {}", idl_path.to_string_lossy()))?;
    let ast = idl_file.parse::<IDLProg>().with_context(|| {
        format!(
            "Failed to parse the Candid file: {}",
            idl_path.to_string_lossy()
        )
    })?;
    let mut env = TypeEnv::new();
    let actor = check_prog(&mut env, &ast).with_context(|| {
        format!(
            "Failed to type check the Candid file: {}",
            idl_path.to_string_lossy()
        )
    })?;
    Ok((env, actor))
}

fn blob_from_arguments(
    arguments: Option<&str>,
    arg_type: &ArgType,
    method_type: &Option<(candid::parser::typing::TypeEnv, candid::types::Function)>,
) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let arguments = if arguments == Some("-") {
        use std::io::Read;
        std::io::stdin().read_to_end(&mut buffer).unwrap();
        std::str::from_utf8(&buffer).ok()
    } else {
        arguments
    };

    match arg_type {
        ArgType::Raw => {
            let bytes = hex::decode(&arguments.unwrap_or(""))
                .context("Argument is not a valid hex string")?;
            Ok(bytes)
        }
        ArgType::Idl => {
            let arguments = arguments.unwrap_or("()");
            let args = arguments.parse::<IDLArgs>();
            let typed_args = match method_type {
                None => args
                    .context("Failed to parse arguments with no method type info")?
                    .to_bytes(),
                Some((env, func)) => {
                    let first_char = arguments.chars().next();
                    let is_candid_format = first_char.map_or(false, |c| c == '(');
                    // If parsing fails and method expects a single value, try parsing as IDLValue.
                    // If it still fails, and method expects a text type, send arguments as text.
                    let args = args.or_else(|e| {
                        if func.args.len() == 1 && !is_candid_format {
                            let is_quote = first_char.map_or(false, |c| c == '"');
                            if candid::types::Type::Text == func.args[0] && !is_quote {
                                Ok(IDLValue::Text(arguments.to_string()))
                            } else {
                                arguments.parse::<IDLValue>()
                            }
                            .map(|v| IDLArgs::new(&[v]))
                        } else {
                            Err(e)
                        }
                    });
                    args.context("Failed to parse arguments with method type info")?
                        .to_bytes_with_types(&env, &func.args)
                }
            }
            .context("Failed to serialize Candid values")?;
            Ok(typed_args)
        }
    }
}

pub fn get_effective_canister_id(
    is_management_canister: bool,
    method_name: &str,
    arg_value: &[u8],
    canister_id: Principal,
) -> Result<Principal> {
    if is_management_canister {
        let method_name = MgmtMethod::from_str(method_name).with_context(|| {
            format!(
                "Attempted to call an unsupported management canister method: {}",
                method_name
            )
        })?;
        match method_name {
            MgmtMethod::CreateCanister | MgmtMethod::RawRand => bail!(
                "{} can only be called via an inter-canister call.",
                method_name.as_ref()
            ),
            MgmtMethod::InstallCode => {
                let install_args = candid::Decode!(arg_value, CanisterInstall)
                    .context("Argument is not valid for CanisterInstall")?;
                Ok(install_args.canister_id)
            }
            MgmtMethod::StartCanister
            | MgmtMethod::StopCanister
            | MgmtMethod::CanisterStatus
            | MgmtMethod::DeleteCanister
            | MgmtMethod::DepositCycles
            | MgmtMethod::UninstallCode
            | MgmtMethod::ProvisionalTopUpCanister => {
                #[derive(CandidType, Deserialize)]
                struct In {
                    canister_id: Principal,
                }
                let in_args =
                    candid::Decode!(arg_value, In).context("Argument is not a valid Principal")?;
                Ok(in_args.canister_id)
            }
            MgmtMethod::ProvisionalCreateCanisterWithCycles => Ok(Principal::management_canister()),
            MgmtMethod::UpdateSettings => {
                #[derive(CandidType, Deserialize)]
                struct In {
                    canister_id: Principal,
                    settings: CanisterSettings,
                }
                let in_args = candid::Decode!(arg_value, In)
                    .context("Argument is not valid for UpdateSettings")?;
                Ok(in_args.canister_id)
            }
        }
    } else {
        Ok(canister_id)
    }
}

fn create_identity(maybe_pem: Option<PathBuf>) -> impl Identity {
    if let Some(pem_path) = maybe_pem {
        BasicIdentity::from_pem_file(pem_path).expect("Could not read the key pair.")
    } else {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .expect("Could not generate a key pair.")
            .as_ref()
            .to_vec();

        BasicIdentity::from_key_pair(
            Ed25519KeyPair::from_pkcs8(&pkcs8_bytes).expect("Could not generate the key pair."),
        )
    }
}

#[derive(Clap)]
enum ArgType {
    Idl,
    Raw,
}

// fn print_result (result : Result<&[u8]>)  {
//         // let arg: ArgType = ArgType::Idl;
//     let output: ArgType = ArgType::Idl;

//     match result {
//         Ok(blob) => {
//             print_idl_blob(&blob, &output, &None)
//                 .context("Failed to print result blob")?;
//         }
//         Err(AgentError::TransportError(_)) => return Ok(()),
//         Err(AgentError::HttpError(HttpErrorPayload {
//             status,
//             content_type,
//             content,
//         })) => {
//             let mut error_message =
//                 format!("Server returned an HTTP Error:\n  Code: {}\n", status);
//             match content_type.as_deref() {
//                 None => error_message
//                     .push_str(&format!("  Content: {}\n", hex::encode(content))),
//                 Some("text/plain; charset=UTF-8") | Some("text/plain") => {
//                     error_message.push_str("  ContentType: text/plain\n");
//                     error_message.push_str(&format!(
//                         "  Content:     {}\n",
//                         String::from_utf8_lossy(&content)
//                     ));
//                 }
//                 Some(x) => {
//                     error_message.push_str(&format!("  ContentType: {}\n", x));
//                     error_message.push_str(&format!(
//                         "  Content:     {}\n",
//                         hex::encode(&content)
//                     ));
//                 }
//             }
//             bail!(error_message);
//         }
//         Err(s) => Err(s).context("Got an error when make the canister call")?,
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    // let opts: Opts = Opts::parse();

    let canister_id = Principal::from_str("jtozz-6yaaa-aaaai-qangq-cai").unwrap();
    let replica = "https://ic0.app";
    // let canister_id = Principal::from_str("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
    // let replica = "http://localhost:8000/";
    // let candid = Some(PathBuf::from("../../src/ic_chatbot/main.did"));
    let candid = Some(PathBuf::from("../src/ic_chatbot/main.did"));

    let agent = Agent::builder()
        .with_transport(
            agent::http_transport::ReqwestHttpReplicaV2Transport::create(replica.clone())
                .context("Failed to create Transport for Agent")?,
        )
        .with_boxed_identity(Box::new(create_identity(None)))
        .build()
        .context("Failed to build the Agent")?;

    fetch_root_key_from_non_ic(&agent, &replica).await?;

    let blocks_json_text = fs::read_to_string("../flow_chart/blocks.json").expect("Something went wrong reading the blocks file");
    let method_name = "load_blocks_from_json".to_string();
    // let arg_value = Some(hex::encode(blocks_json_text));
    let arg_value = Some(blocks_json_text);

    let mut builder = agent.update(&canister_id, &method_name);

    let maybe_candid_path = candid.as_ref();
    let method_type = match maybe_candid_path {
        None => None,
        Some(path) => get_candid_type(&path, &method_name)
            .context("Failed to get method type from candid file")?,
    };
    let arg = ArgType::Idl;
    let arg = blob_from_arguments(arg_value.as_deref(), &arg, &method_type)
                .context("Invalid arguments")?;
            
    println!("Sending block information to ic_chatbot");
    eprint!(".");
    let result = builder
        .with_arg(arg)
        .with_effective_canister_id(canister_id)
        .call_and_wait(
            garcon::Delay::builder()
                .exponential_backoff(std::time::Duration::from_secs(1), 1.1)
                .side_effect(|| {
                    eprint!(".");
                    Ok(())
                })
                .timeout(std::time::Duration::from_secs(60 * 5))
                .build(),
        )
        .await;
    eprintln!("Result : {:?}", result);




    let paths = fs::read_dir("../flow_chart/intents").unwrap();

    let mut json_obj = json::JsonValue::new_object();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();

        let json_text = fs::read_to_string(path.clone()).expect("Something went wrong reading the intent file");
        let parsed = json::parse(&json_text).unwrap();
        json_obj.insert(path, parsed);
    }
    let intents_json_text = json_obj.dump();

    let method_name = "load_intents_from_json".to_string();
    let arg_value = Some(intents_json_text);
    let arg = ArgType::Idl;
    let method_type = match maybe_candid_path {
        None => None,
        Some(path) => get_candid_type(&path, &method_name)
            .context("Failed to get method type from candid file")?,
    };
    let arg = blob_from_arguments(arg_value.as_deref(), &arg, &method_type)
                .context("Invalid arguments")?;
            
    let mut builder = agent.update(&canister_id, &method_name);
    
    println!("Sending intent information to ic_chatbot");
    eprint!(".");
    let result = builder
        .with_arg(arg)
        .with_effective_canister_id(canister_id)
        .call_and_wait(
            garcon::Delay::builder()
                .exponential_backoff(std::time::Duration::from_secs(1), 1.1)
                .side_effect(|| {
                    eprint!(".");
                    Ok(())
                })
                .timeout(std::time::Duration::from_secs(60 * 5))
                .build(),
        )
        .await;
    eprintln!("Result : {:?}", result);



    // println!("{:?}", json_text);
    // let principal: ic_cdk::export::Principal = ic_cdk::export::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    // let (result,) : (String,) = ic_cdk::call(principal, "load_intents_from_json", (json_text,)).await.unwrap();






    // print_result(result);
    Ok(())
}