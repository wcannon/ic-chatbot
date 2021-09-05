use std::collections::HashMap;
use std::cell::{Cell, RefCell};  
use crate::intent::{*};
use crate::block::{*};
use crate::factory::{*};
// use dfn_core::{api::trap_with, over, over_async, stable};
use ic_cdk_macros::post_upgrade;

pub type SessionId = String;
const INTENT_DIR: &str = "/Users/satya/work/hackathon/botmock-dialogflow-export/output/upload/intents";
const BLOCK_FILE: &str = "/Users/satya/work/hackathon/botmock-dialogflow-export/output/webhook/blocks.json"; 

thread_local! {
  static STATE: State = State {
      blocks : RefCell::new(Vec::new()),
      intents : RefCell::new(Vec::new()),
      start_block : Box::new(StartBlock::new()),
      session_info : RefCell::new(HashMap::new())
  }
}

struct State {
	blocks : RefCell<Vec<Box<dyn Block>>>,
	intents : RefCell<Vec<Box<dyn Intent>>>,
	start_block : Box<dyn Block>,
	session_info : RefCell<HashMap<SessionId, Session>>
}

impl State {
	pub fn push_block (& self, block : Box<dyn Block>) {
		self.blocks.borrow_mut().push(block);
	}

	pub fn push_intent (& self, intent : Box<dyn Intent>) {
		self.intents.borrow_mut().push(intent);
	}

	pub fn set_start_block (&mut self, block : Box<dyn Block>) {
		self.start_block = Box::new(StartBlock::new()); 
		// .set(block); 
		// self.start_block.copy_from(Box::into_raw(block));
		//self.start_block.replace(block);
		// let start_block = self.start_block.borrow_mut();
		// start_block = block;
	}

}

enum SequenceElement {
	VisitedBlock(Box<dyn Block>),
	TriggeredIntent(Box<dyn Intent>),
	UserInput(String)
}

struct Session {
	session_id : SessionId,
	visited_sequence : Vec<SequenceElement>
}

impl Session {

	pub fn new() -> Self {	//Initializes latest_block with StartBlock
		let mut visited_sequence : Vec<SequenceElement> = Vec::new();
		// let start_block = STATE.with (|s| s.start_block);
		// let start_block : Box<dyn Block> = start_block;
		
		// STATE.with(|s| {
		// 				for block in s.blocks.borrow().iter() {
		// 					if block.get_component_type() == "start_block" {
		// 						let start_block = block.clone(); 
		// 						visited_sequence.push( SequenceElement::VisitedBlock(*start_block) );
		// 						break;
		// 					}
		// 				}
		// 			}); 
		// visited_sequence.push( SequenceElement::VisitedBlock(start_block) );

		Session {
			session_id : Session::gen_new_session_id(),
		 	visited_sequence : visited_sequence
		}
	}

	fn gen_new_session_id () -> SessionId {
		String::from("new_session")
	}

	// pub fn get_session (session_id : SessionId) -> Option<&Session> {
	// 	STATE.with (|s| { s.session_info.get(&session_id) } ); 
	// } 

	fn process_user_input(&mut self, user_input : String) -> String {
		String::new()
	}
}

#[post_upgrade]
fn initialize_state() {
	let (blocks, intents) = FactoryImpl::load_json_files(INTENT_DIR, BLOCK_FILE);
					
	STATE.with(|s| {
					for block in blocks {
						s.push_block(block); //blocks;
					}
					for intent in intents {
						s.push_intent(intent); //blocks;
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