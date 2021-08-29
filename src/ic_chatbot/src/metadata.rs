use std::collections::HashMap;
use std::any::type_name;

pub trait MetaData {
	fn from_json(json_text : &str) -> Self;

	// fn load_from_json(&mut self, json_text : &str);

	// fn convert_to_json(&self) -> String;
	
	// fn perform_action(&mut self);
}

#[derive(PartialEq, Eq, Debug)]
pub struct NextBlockInfo {
	next_block : HashMap<String, String>
}

impl MetaData for NextBlockInfo {
	fn from_json(json_text : &str) -> Self {
		let parsed = json::parse(json_text).unwrap();
		
		// println!("{:#?}", parsed);

		let mut next_block = HashMap::<String, String>::new();
		for key_value_pair in parsed.members() {
			let mut iter = key_value_pair.entries(); 
			let key = iter.next().unwrap().1.to_string();
			let value = iter.next().unwrap().1.to_string(); 
			// println!("{:?}, {:?}", key, value); 
			next_block.insert(key, value); 
		}
		
		NextBlockInfo {
			next_block : next_block
		}
	}
}