// use ic_types::*;
// use ic_util::*;
// use ic_fondue::{ic_manager::IcHandle, internet_computer::InternetComputer};
// use ic_registry_subnet_type::SubnetType;
// use ic_utils::interfaces::ManagementCanister;

use ic_cdk::api::call::call;
use ic_cdk::export::candid::{Deserialize, Func, Principal};
use ic_cdk::api::{caller, data_certificate, id, set_certified_data, time, trap};
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
// use candid::de::ArgumentDecoder;
// use candid::ser::ArgumentEncoder;
use candid::{decode_args, encode_args};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use candid::{*};
use ic_cdk::{*};
use candid::utils::{*};
use candid::de::IDLDeserialize;
use candid::ser::IDLBuilder;
use candid::{CandidType, Result};
// use crate::CanisterId;
// use serde::de::Deserialize;
use std::io;
use std::fs;
use json;

const BLOCK_FILE: &str = "../flow_chart/blocks.json"; 
const INTENT_DIR: &str = "../flow_chart/intents"; 

async fn load_blocks() {  
  let blocks_json_text = fs::read_to_string(BLOCK_FILE).expect("Something went wrong reading the blocks file");
  let principal: ic_cdk::export::Principal = ic_cdk::export::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
  let (result,) : (String,) = ic_cdk::call(principal, "load_blocks_from_json", (blocks_json_text,)).await.unwrap(); 
}

async fn load_intents() {
    let paths = fs::read_dir(INTENT_DIR).unwrap();

    let mut result = json::JsonValue::new_object();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();

        let json_text = fs::read_to_string(path.clone()).expect("Something went wrong reading the intent file");
        let parsed = json::parse(&json_text).unwrap();
        result.insert(path, parsed);
    }
    let json_text = result.dump();
    // println!("{:?}", json_text);
    let principal: ic_cdk::export::Principal = ic_cdk::export::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let (result,) : (String,) = ic_cdk::call(principal, "load_intents_from_json", (json_text,)).await.unwrap();
}

fn main() {
    load_blocks();
    load_intents();
}
