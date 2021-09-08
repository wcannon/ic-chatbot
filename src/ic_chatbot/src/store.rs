use std::collections::HashMap;
use std::cell::{Cell, RefCell};  
use crate::intent::{*};
use crate::block::{*};
use crate::factory::{*};
// use dfn_core::{api::trap_with, over, over_async, stable};
use ic_cdk_macros::init;
pub use crate::types::{*};
use std::convert::TryInto;
use ic_cdk_macros::post_upgrade;
// use ic_test_utilities::universal_canister::{call_args, wasm};
// use ic_types::{
//     ic00,
//     ic00::{EmptyBlob, Method},
//     ingress::WasmResult,
//     messages::MAX_INTER_CANISTER_PAYLOAD_IN_BYTES,
//     time::current_time_and_expiry_time,
//     user_error::ErrorCode,
//     CanisterId, NumBytes, RegistryVersion,
// };

use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Func, Principal};
use ic_cdk::api::{caller, data_certificate, id, set_certified_data, time, trap};

const INTENT_DIR: &str = "../../flow_chart/intents";
const BLOCK_FILE: &str = "../../flow_chart/blocks.json"; 

// trait Btemp : (Block + Clone);
thread_local! {
  static STATE: State = State {
      // blocks : RefCell::new(Vec::new()),
      blocks : RefCell::new(HashMap::new()),
      // intents : RefCell::new(Vec::new()),
      intents : RefCell::new(HashMap::new()),
      // start_block : Box::new(StartBlock::new()),
      session_info : RefCell::new(HashMap::new())
  }
}

#[derive(Clone)]
struct State {
	// blocks : RefCell<Vec<Box<dyn Block>>>,
	// intents : RefCell<Vec<Box<dyn Intent>>>,
	blocks : RefCell<HashMap<NodeName, Box<dyn Block>>>,
	intents : RefCell<HashMap<NodeName, Box<dyn Intent>>>,
	// start_block : Box<dyn Block>,
	session_info : RefCell<HashMap<SessionId, RefCell<Session>>>
}

impl State {
	pub fn push_block (& self, block : Box<dyn Block>) {
		println!("Indexing {}", block.get_node_name());
		self.blocks.borrow_mut().insert(block.get_node_name().to_string(), block);
	}

	pub fn push_intent (& self, intent : Box<dyn Intent>) {
		// let b1 = Box::new(IntentImpl::new());
		// let mut b2 = Box::new(IntentImpl::new());
		// b2 = b1; 
		// println!("{}", b2); 
		self.intents.borrow_mut().insert(intent.get_intent_name().to_string(), intent);
	}

	// pub fn set_start_block (&mut self, block : Box<dyn Block>) {
	// 	self.start_block = Box::new(StartBlock::new()); 
	// 	// .set(block); 
	// 	// self.start_block.copy_from(Box::into_raw(block));
	// 	//self.start_block.replace(block);
	// 	// let start_block = self.start_block.borrow_mut();
	// 	// start_block = block;
	// }
} 


#[derive(Clone)]
enum SequenceElement {
	VisitedBlock(Box<dyn Block>),
	TriggeredIntent(Box<dyn Intent>),
	TriggeredWrongInput,
	TriggeredEndOfChart,
	UserInput(String)
}






#[derive(Clone)]
pub struct Session {
	session_id : SessionId,
	visited_sequence : Vec<SequenceElement>
}

impl Session {
	// ctr : static i32 = 0;
	pub fn new() -> Self {	//Initializes latest_block with StartBlock
		let mut visited_sequence : Vec<SequenceElement> = Vec::new();
		let start_block = STATE.with (|s| (s.blocks.borrow().get(&"StartBlock".to_string()).expect("Unable to find start block in the state").clone()) );
		visited_sequence.push( SequenceElement::VisitedBlock(start_block) );

		let session = Session {
			session_id : Session::gen_new_session_id(),
		 	visited_sequence : visited_sequence
		};
		STATE.with(|s| s.session_info.borrow_mut().insert(session.get_session_id().to_string(), RefCell::new(session.clone()) ));
		session
	}

	pub fn process_user_input(session_id : SessionId, user_input : String) -> json::JsonValue {
		println!("Processing session : \"{}\"", session_id); 
		if STATE.with(|s| s.session_info.borrow().contains_key(&session_id)) {
			STATE.with(|s| s.session_info.borrow_mut().get(&session_id).expect("Invalid session id").borrow_mut().process_user_input_for_session(user_input))
		}
		else {
			// println!("{}", session);
			let mut result = json::JsonValue::new_object();
			result["component_type"] = "Error".into();
			result["text"] = "Invalid session id ".into();
			result
		}

	}


	// fn gen_rand_vector() -> [u8; 32] {
	// 	let res = match call(Principal::management_canister(), "raw_rand", ()) {
	//         					Ok((res,)) => res,
	//         					Err((_, err)) => trap(&format!("failed to get salt: {}", err)),
 //    						};
 //    	let salt: [u8; 32] = res[..].try_into().unwrap_or_else(|_| {
 //        trap(&format!(
 //            "expected raw randomness to be of length 32, got {}",
 //            res.len()
 //        	));
	//     });
 //    	salt
	// }

	// async fn gen_rand_vector() -> [u8; 32] {
	// 	let (res,) = call(Principal::management_canister(), "raw_rand", ()).await.unwrap();
 //    	res.into()
 //    	// let salt: [u8; 32] = res[..].try_into().unwrap_or_else(|_| {
 //     //    trap(&format!(
 //     //        "expected raw randomness to be of length 32, got {}",
 //     //        res.len()
 //     //    	));
	//     // });
 //    	// salt
	// }


 	fn gen_new_session_id () -> SessionId {
		// let rand_vec : [u8;32] = Session::gen_rand_vector();
		// std::str::from_utf8(&rand_vec).unwrap().to_string()
		String::from("sample_session_id")
	}

	pub fn convert_to_json (&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();
		data["component_type"] = "session_id".into(); 
		data["session_id"] = self.session_id.clone().into(); 
		data
	}

	// pub fn get_session (session_id : SessionId) -> Option<&Session> {
	// 	STATE.with (|s| { s.session_info.get(&session_id) } ); 
	// } 

	pub fn get_session_id (&self) -> &SessionId {
		&self.session_id
	}

	fn get_last_visited_block(&self) -> Box<dyn Block> {
		assert!(self.visited_sequence.len() > 0); 
		let mut index = self.visited_sequence.len()-1; 
		while (index >= 0) {
			match &self.visited_sequence[index] {
				SequenceElement::VisitedBlock(block) => {return block.clone();},
				SequenceElement::TriggeredIntent(_) => panic!("Last visited element is an intent"),
				SequenceElement::TriggeredWrongInput => {index = index - 1;},
				SequenceElement::TriggeredEndOfChart => {index = index - 1;}
				SequenceElement::UserInput(_) => panic!("Last visited element is a user input")
			};
		};
		panic!("No blocks left in visited sequence");
	}

	pub fn process_user_input_for_session(&mut self, user_input : String) -> json::JsonValue {
		let mut last_block = self.get_last_visited_block();
		let mut result_path = json::JsonValue::new_array();
		
		if !user_input.is_empty() {
			self.visited_sequence.push( SequenceElement::UserInput(user_input.clone()) );
		}
		
		while ((!user_input.is_empty()) || (last_block.can_perform_action_on_empty_input())) {
			let (link, intent, nodename) = last_block.perform_action(&user_input, &STATE.with(|s| s.intents.clone()));
			if (link == LinkType::intent) {
				self.visited_sequence.push( SequenceElement::TriggeredIntent(STATE.with(|s| s.intents.borrow().get(&intent).unwrap().clone())) );
			}
			if (link == LinkType::endofchart) {
				self.visited_sequence.push( SequenceElement::TriggeredEndOfChart );
				break;
				//TODO: set last_block = default_block 
			}
			else if (link == LinkType::wronginput) {
				self.visited_sequence.push( SequenceElement::TriggeredWrongInput );
				break;
			}
			else {
				let mut error_msg = "Unable to find the next node ".to_string();
				error_msg.push_str(&last_block.get_node_name());
				error_msg.push_str(",");
				error_msg.push_str(&nodename);
				error_msg.push_str(",");
				error_msg.push_str(&intent);
				error_msg.push_str(",");
				error_msg.push_str(&link.to_str());
				last_block = STATE.with(|s| s.blocks.borrow().get(&nodename).expect(&error_msg).clone());
				println!("Pushing block : {}", last_block.get_node_name());
				self.visited_sequence.push( SequenceElement::VisitedBlock(last_block.clone()) );
				result_path.push(last_block.convert_to_json()); 

				if !last_block.can_perform_action_on_empty_input() {
					break;
				}
			}
		}
		result_path
	}
}




pub fn store_blocks_in_state (blocks: Vec::<Box<dyn Block>>) {
	STATE.with(|s| {
		for block in blocks {
			s.push_block(block);
		}
	});
}


pub fn store_intents_in_state (intents: Vec::<Box<dyn Intent>>) {
	STATE.with(|s| {
		for intent in intents {
			s.push_intent(intent);
		}
	});
}

pub fn summarize_all_blocks () -> json::JsonValue {
	let mut result = json::JsonValue::new_array(); 
	STATE.with(|s| {
		for (nodename, block) in s.blocks.borrow().iter() {
			result.push(block.convert_to_json());
		}
	});
	result
}


#[init]
pub fn initialize_state() {
	// use std::env;
 //    let path = env::current_dir();
 //    println!("The current directory is {}", path.unwrap().display());
    // panic!("The code panics");
	let (blocks, intents) = FactoryImpl::load_json_files(INTENT_DIR, BLOCK_FILE);
	
	STATE.with(|s| {
					for block in blocks {
						s.push_block(block); //blocks;
					}
					for intent in intents {
						s.push_intent(intent); //intents;
					}
					// // s.intents = intents;
					// for block in blocks {
					// 	if block.get_component_type() == "start_block" {
					// 		s.set_start_block(block);
					// 	}
					// }
				}
			);
}

#[post_upgrade]
fn reset_state() {
	STATE.with(|s| {
					s.blocks.borrow_mut().clear();
					s.intents.borrow_mut().clear();
					s.session_info.borrow_mut().clear();
					});
}

// #[post_upgrade]
// fn initialize_state() {
	// use std::env;
 //    let path = env::current_dir();
 //    println!("The current directory is {}", path.unwrap().display());
        
	// let (blocks, intents) = FactoryImpl::load_json_files(INTENT_DIR, BLOCK_FILE);
	
	// STATE.with(|s| {
	// 				for block in blocks {
	// 					s.push_block(block); //blocks;
	// 				}
	// 				for intent in intents {
	// 					s.push_intent(intent); //blocks;
	// 				}
	// 				// // s.intents = intents;
	// 				// for block in blocks {
	// 				// 	if block.get_component_type() == "start_block" {
	// 				// 		s.set_start_block(block);
	// 				// 	}
	// 				// }
	// 			}
	// 		);
// }
