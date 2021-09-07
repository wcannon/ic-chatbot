pub use crate::metadata::{*};
pub use crate::quickreply::{*};
pub use crate::button::{*};
pub use crate::types::{*};
pub use crate::intent::{*};
use std::collections::HashMap;
use std::cell::{Cell, RefCell};  


pub trait Block : BlockClone {

	fn from_json(&mut self, json_text : &str);

	fn convert_to_json(&self) -> json::JsonValue;
	
	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (IntentName, BlockName);

	//Getter Methods 
	fn get_id(&self) -> &str; 

	fn get_component_type(&self) -> &str; 

	fn get_node_name(&self) -> &str; 

	// fn get_delay(&self) -> &u16; 

	// fn get_end_conversation(&self) -> &bool; 
}

pub trait BlockClone {
	fn clone_box(&self) -> Box<dyn Block>; 
}

impl<T> BlockClone for T
where 
	T : 'static + Block + Clone,
{
	fn clone_box(&self) -> Box<dyn Block> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Block> {
	fn clone(&self) -> Box<dyn Block> {
		self.clone_box()
	}
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct TextBlock {
	id : String,
	component_type : String,
	node_name : String,
	next_block_info : NextBlockInfo,
	delay : u16,
	end_conversation : bool,

	text : String,
	alternate_replies : Vec<String>
}

impl TextBlock {
	pub fn new() -> Self {
		TextBlock { 
			id : String::new(),
			component_type : String::new(),
			node_name : String::new(),
			next_block_info : NextBlockInfo::new(),
			delay : 0,
			end_conversation : false,
			text : String::new(),
			alternate_replies : Vec::<String>::new()
		}
	}

	pub fn static_block() -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = "text".into();
		data["text"] = "Internet Computer is Cloud 3.0.".into();
		data["delay"] = 500.into();
		data["end_conversation"] = false.into();
		let mut alt_replies : Vec<String> = Vec::new();
		alt_replies.push("Internet Computer is a next generation cloud service.".to_string()); 
		alt_replies.push("Internet Computer is developed by Dfinity foundation.".to_string()); 
		data["alternate_replies"] = alt_replies.into();
		// data.dump()
		data
	}
}

impl Block for TextBlock {
	
	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();		
		assert_eq!(parsed["component_type"], "text"); 

		let mut alternate_replies : Vec<String> = Vec::<String>::new();
		for alternate_reply in parsed["alternate_replies"].members() {
			alternate_replies.push(alternate_reply.to_string());
		}
		
		self.id 			= parsed["id"].to_string();
		self.component_type = parsed["component_type"].to_string();
		self.node_name 		= parsed["nodeName"].to_string();
		self.next_block_info.from_json(&parsed["metadata"].to_string());
		self.text 			= parsed["text"].to_string();
		self.delay 			= parsed["delay"].as_u16().unwrap();
		self.end_conversation = parsed["end_conversation"].as_bool().unwrap();
		self.alternate_replies = alternate_replies;
	}

	fn convert_to_json(&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = self.component_type.clone().into();
		data["text"] = self.text.clone().into();
		data["delay"] = self.delay.into();
		data["end_conversation"] = self.end_conversation.into();
		data["alternate_replies"] = self.alternate_replies.clone().into();
		data
	}
	
	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (IntentName, BlockName) {
		self.next_block_info.get_next_block(user_input, intents)
	}

	//Getter methods
	fn get_id(&self) -> &str {
		&self.id
	}

	fn get_component_type(&self) -> &str {
		&self.component_type
	}

	fn get_node_name(&self) -> &str {
		&self.node_name
	}

	// fn get_delay(&self) -> &u16 {
	// 	&self.delay
	// }

	// fn get_end_conversation(&self) ->  &bool {
	// 	&self.end_conversation
	// }

}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct StartBlock {
	id : String,
	component_type : String,
	node_name : String,
	next_block_info : NextBlockInfo
}

impl StartBlock {
	pub fn new() -> Self {
		StartBlock {
			id : String::new(),
			component_type : String::new(),
			node_name : String::new(),
			next_block_info : NextBlockInfo::new()
		}
	}
}

impl Block for StartBlock {
	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();		
		assert_eq!(parsed["component_type"], "start_block"); 

		self.id 			= parsed["id"].to_string();
		self.component_type = parsed["component_type"].to_string();
		self.node_name 		= parsed["nodeName"].to_string();
		self.next_block_info.from_json(&parsed["metadata"].to_string());
	}

	fn convert_to_json(&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = self.component_type.clone().into();
		// data.dump()
		data
	}

	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (IntentName, BlockName) {
		self.next_block_info.get_next_block(user_input, intents)
	}


	//Getter methods
	fn get_id(&self) -> &str {
		&self.id
	}

	fn get_component_type(&self) -> &str {
		&self.component_type
	}

	fn get_node_name(&self) -> &str {
		&self.node_name
	}
}








#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct QuickRepliesBlock {
	id : String,
	component_type : String,
	node_name : String,
	next_block_info : NextBlockInfo,
	delay : u16,
	end_conversation : bool,

	text : String,
	quick_replies : Vec<QuickReplyImpl>
}

impl QuickRepliesBlock {
	pub fn new() -> Self {
		QuickRepliesBlock { 
			id : String::new(),
			component_type : String::new(),
			node_name : String::new(),
			next_block_info : NextBlockInfo::new(),
			end_conversation : false,
			delay : 0,
			
			text : String::new(),
			quick_replies : Vec::<QuickReplyImpl>::new()
		}
	}

	pub fn static_block() -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = "quick_replies".into();
		data["text"] = "Does this answer your question?".into();
		data["delay"] = 500.into();
		data["end_conversation"] = false.into();
		let mut replies = json::JsonValue::new_array();
		replies.push(QuickReplyImpl::static_block("yes".to_string()));
		replies.push(QuickReplyImpl::static_block("no".to_string()));
		data["quick_replies"] = replies.into(); 
		data
	}
}

impl Block for QuickRepliesBlock {
	
	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();		
		assert_eq!(parsed["component_type"], "quick_replies"); 
		
		self.id 			= parsed["id"].to_string();
		self.component_type = parsed["component_type"].to_string();
		self.node_name 		= parsed["nodeName"].to_string();
		self.next_block_info.from_json(&parsed["metadata"].to_string());
		self.text 			= parsed["text"].to_string();
		self.delay 			= parsed["delay"].as_u16().unwrap();
		self.end_conversation = parsed["end_conversation"].as_bool().unwrap();
	}

	fn convert_to_json(&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		let mut replies = json::JsonValue::new_array();
		for quick_reply in self.quick_replies.iter() {
			replies.push(quick_reply.convert_to_json());
		}

		data["component_type"] = self.component_type.clone().into();
		data["text"] = self.text.clone().into();
		data["delay"] = self.delay.into();
		data["end_conversation"] = self.end_conversation.into();
		data["quick_replies"] = replies.into();
		data
	}
	
	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (IntentName, BlockName) {
		self.next_block_info.get_next_block(user_input, intents)
	}

	//Getter methods
	fn get_id(&self) -> &str {
		&self.id
	}

	fn get_component_type(&self) -> &str {
		&self.component_type
	}

	fn get_node_name(&self) -> &str {
		&self.node_name
	}

	// fn get_delay(&self) -> &u16 {
	// 	&self.delay
	// }

	// fn get_end_conversation(&self) ->  &bool {
	// 	&self.end_conversation
	// }

}





#[derive(PartialEq, Eq, Debug, Default)]
pub struct ButtonBlock {
	id : String,
	component_type : String,
	node_name : String,
	next_block_info : NextBlockInfo,
	delay : u16,
	end_conversation : bool,

	text : String,
	buttons : Vec<ButtonImpl>
}

impl ButtonBlock {
	pub fn new() -> Self {
		ButtonBlock { 
			id : String::new(), 
			component_type : String::new(),
			node_name : String::new(),
			next_block_info : NextBlockInfo::new(),
			delay : 0, 
			end_conversation : false, 
			text : String::new(),
			buttons : Vec::<ButtonImpl>::new()
		}
	}

	pub fn static_block() -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = "buttons".into();
		data["text"] = "Here are some articles that you requested.".into();
		data["delay"] = 500.into();
		data["end_conversation"] = false.into();
		let mut buttons = json::JsonValue::new_array();
		buttons.push(ButtonImpl::static_block());
		data["buttons"] = buttons.into(); 
		data
	}
}