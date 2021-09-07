use std::collections::HashMap;
use std::cell::{Cell, RefCell};  
use crate::intent::{*};
use crate::block::{*};
// use crate::factory::{*};
// use dfn_core::{api::trap_with, over, over_async, stable};
use ic_cdk_macros::post_upgrade;
pub use crate::types::{*};
use std::convert::TryInto;
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

struct State {
	// blocks : RefCell<Vec<Box<dyn Block>>>,
	// intents : RefCell<Vec<Box<dyn Intent>>>,
	blocks : RefCell<HashMap<NodeName, Box<dyn Block>>>,
	intents : RefCell<HashMap<NodeName, Box<dyn Intent>>>,
	// start_block : Box<dyn Block>,
	session_info : RefCell<HashMap<SessionId, Session>>
}

impl State {
	pub fn push_block (& self, block : Box<dyn Block>) {
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

enum SequenceElement {
	VisitedBlock(Box<dyn Block>),
	TriggeredIntent(Box<dyn Intent>),
	UserInput(String)
}







pub struct Session {
	session_id : SessionId,
	visited_sequence : Vec<SequenceElement>
}

impl Session {
	// ctr : static i32 = 0;
	pub fn new() -> Self {	//Initializes latest_block with StartBlock
		let mut visited_sequence : Vec<SequenceElement> = Vec::new();
		let start_block = STATE.with (|s| (s.blocks.borrow().get(&"StartBlock".to_string()).unwrap().clone()) );
		visited_sequence.push( SequenceElement::VisitedBlock(start_block) );

		Session {
			session_id : Session::gen_new_session_id(),
		 	visited_sequence : visited_sequence
		}
	}

	// fn gen_rand_vector() -> [u8; 32] {
		// let res = match call(Principal::management_canister(), "raw_rand", ()) {
	 //        					Ok((res,)) => res,
	 //        					Err((_, err)) => trap(&format!("failed to get salt: {}", err)),
  //   						};
  //   	let salt: [u8; 32] = res[..].try_into().unwrap_or_else(|_| {
  //       trap(&format!(
  //           "expected raw randomness to be of length 32, got {}",
  //           res.len()
  //       	));
	 //    });
    	// salt
	// }

 	fn gen_new_session_id () -> SessionId {
		// let rand_vec : [u8;32] = Session::gen_rand_vector();
		// std::str::from_utf8(&rand_vec).unwrap().to_string()
		String::from("new_session")
	}

	pub fn convert_to_json (&self) -> JsonText {
		let mut data = json::JsonValue::new_object();
		data["component_type"] = "session_id".into(); 
		data["session_id"] = self.session_id.clone().into(); 
		data.dump()
	}

	// pub fn get_session (session_id : SessionId) -> Option<&Session> {
	// 	STATE.with (|s| { s.session_info.get(&session_id) } ); 
	// } 

	fn get_last_visited_block(&self) -> Box<dyn Block> {
		assert!(self.visited_sequence.len() > 0); 
		let last_visited_block = match self.visited_sequence.last().unwrap() {
			SequenceElement::VisitedBlock(block) => block,
			SequenceElement::TriggeredIntent(_) => panic!("Last visited element is an intent"), 
			SequenceElement::UserInput(_) => panic!("Last visited element is a user input")
		};
		last_visited_block.clone()
	}

	fn process_user_input(&mut self, user_input : String) -> String {
		let mut last_block = self.get_last_visited_block();
		let mut result_path = json::JsonValue::new_array();
		self.visited_sequence.push( SequenceElement::UserInput(user_input.clone()) );
			
		loop {
			let (link, intent, nodename) = last_block.perform_action(&user_input, &STATE.with(|s| s.intents.clone()));
			if (link == LinkType::intent) {
				self.visited_sequence.push( SequenceElement::TriggeredIntent(STATE.with(|s| s.intents.borrow().get(&intent).unwrap().clone())) );
			}
			last_block = STATE.with(|s| s.blocks.borrow().get(&nodename).unwrap().clone());
			self.visited_sequence.push( SequenceElement::VisitedBlock(last_block.clone()) );
			result_path.push(last_block.convert_to_json()); 

			if !last_block.can_perform_action_on_empty_input() {
				break;
			}
		}

		// while last_block.can_perform_action_on_empty_input() {
			
		// 	let (IntentName, BlockName) = last_block.perform_action(&user_input, &STATE.with(|s| s.intents.clone())); 

		// }

		// assert!(last_block == SequenceElement::VisitedBlock(_)); 	

		String::new()
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


#[post_upgrade]
fn initialize_state() {
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
}
