use std::collections::HashMap;
use std::cell::{Cell, RefCell};  
use crate::intent::{*};
use crate::types::{*};

pub trait MetaData {
	fn from_json(&mut self, json_text : &str);
	fn get_next_block(&self, user_input : &String, intents : &RefCell<HashMap<String, Box<dyn Intent>>>) -> (IntentName, BlockName); 
	// fn convert_to_json(&self) -> json::JsonValue; 
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct NextBlockInfo {
	next_block : HashMap<IntentName, BlockName>
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

	fn get_next_block(&self, user_input : &String, intents : &RefCell<HashMap<String, Box<dyn Intent>>>) -> (IntentName, BlockName) { 
		if (self.next_block.contains_key("Default")) {
			return ("Default".to_string(), self.next_block.get("Default").unwrap().clone()); 
		}
		let mut intent_scores = HashMap::<String, usize>::new();
		for (key, val) in self.next_block.iter() {
			assert!(key.starts_with("Intent-"), "Metadata doesn't start with Intent-");
			let intent_name = key.trim_start_matches("Intent-");
			let intent = intents.borrow().get(&intent_name.to_string()).expect("Intent in metadata doesn't exist").clone(); 
			let score = intent.get_matching_score(&user_input);
			intent_scores.insert(intent_name.to_string(), score);
		}
		let min_score = *intent_scores.values().cloned().collect::<Vec<usize>>().iter().min().unwrap();
		let intent = intent_scores.iter().find_map(|(key, &val)| if val == min_score { Some(key) } else { None }).unwrap();
		(intent.to_string(), self.next_block.get(intent).unwrap().clone())
	}

	// fn convert_to_json(&self) -> json::JsonValue {
	// 	json::JsonValue::new_object()
	// }
}






