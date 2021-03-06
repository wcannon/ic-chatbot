use std::collections::HashMap;
use std::cell::{RefCell};  
use crate::intent::{*};
use crate::types::{*};



pub trait MetaData {
	fn from_json(&mut self, json_text : &str);
	fn get_next_block(&self, user_input : &String, intents : &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName); 

	//Returns true, if we can jump to next block even on no input from user. 
	fn can_perform_action_on_empty_input(&self) -> bool;	
	// fn convert_to_json(&self) -> json::JsonValue; 
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct NextBlockInfo {
	next_block : HashMap<InputName, BlockName>	//InputName, BlockName are Strings. 
}

impl NextBlockInfo {
	pub fn new() -> Self {
		NextBlockInfo {
			next_block : HashMap::new()
		}
	}
}

impl MetaData for NextBlockInfo {

	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();
		// println!("{:#?}", parsed);
		// let mut next_block = HashMap::<String, String>::new();
		for key_value_pair in parsed.members() {
			let mut iter = key_value_pair.entries(); 
			let key = iter.next().unwrap().1.to_string();
			let value = iter.next().unwrap().1.to_string(); 
			// println!("{:?}, {:?}", key, value); 
			self.next_block.insert(key, value); 
		}
	}

	fn get_next_block(&self, user_input : &String, intents : &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName) { 
		if self.next_block.len() == 0 {
			return (LinkType::Endofchart, String::new(), String::new()); 
		}
		if self.next_block.contains_key("Default") {
			return (LinkType::Defaultlink, String::new(), self.next_block.get("Default").unwrap().clone()); 
		}
		let mut intent_scores = HashMap::<String, usize>::new();
		for (key, val) in self.next_block.iter() {
			if key.starts_with("Intent-") {
				let intent_name = key.trim_start_matches("Intent-");
				let mut error_msg = "Intent in metadata doesn't exist ".to_string();
				error_msg.push_str(intent_name);
				let intent = intents.borrow().get(&intent_name.to_string()).expect(&error_msg).clone(); 
				let score = intent.get_matching_score(&user_input);
				intent_scores.insert(intent_name.to_string(), score);
			}
			else if key.starts_with("Response-") {
				let response = key.trim_start_matches("Response-");
				if user_input.to_lowercase().trim() == response.to_lowercase().trim() {
					return (LinkType::Response, String::new(), val.clone()); 
				}
			}
			else {
				panic!("Metadata should start with Intent or Response");
			}
		}

		//If at least a few links with intents exist, choose the best intent 
		if intent_scores.len() > 0 {
			let min_score = *intent_scores.values().cloned().collect::<Vec<usize>>().iter().min().unwrap();
			let intent = intent_scores.iter().find_map(|(key, &val)| if val == min_score { Some(key) } else { None }).unwrap();
			let mut prefixed_intent = "Intent-".to_string();
			prefixed_intent.push_str(&intent.clone()); 
			(LinkType::Intent, intent.to_string(), self.next_block.get(&prefixed_intent).unwrap().clone())
		} 
		else {	//If there is no link with an intent, and if the user input doesn't match with any response, send nolink. 
			return (LinkType::Wronginput, String::new(), String::new()); 
		}
	}

	//If there is an outlink from the block with "Default" tag, then no user input is required to proceed. 
	fn can_perform_action_on_empty_input(&self) -> bool {
		self.next_block.contains_key("Default")
	}
	
	// fn convert_to_json(&self) -> json::JsonValue {
	// 	json::JsonValue::new_object()
	// }
}






