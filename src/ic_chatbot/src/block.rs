pub use crate::metadata::{*};
pub use crate::quickreply::{*};
pub use crate::button::{*};
pub use crate::types::{*};
pub use crate::intent::{*};
use std::collections::HashMap;
use std::cell::{RefCell};  


pub trait Block : BlockClone {

	fn from_json(&mut self, json_text : &str);

	fn convert_to_json(&self) -> json::JsonValue;
	
	fn can_perform_action_on_empty_input(&self) -> bool;

	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName);

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
	
	fn can_perform_action_on_empty_input(&self) -> bool {	
		self.next_block_info.can_perform_action_on_empty_input()
	}

	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName) {
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
	
	fn can_perform_action_on_empty_input (&self) -> bool {
		self.next_block_info.can_perform_action_on_empty_input()
	}

	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName) {
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
		
		for member in parsed["quick_replies"].members() {
			let mut quick_reply = QuickReplyImpl::new();
			quick_reply.from_json(&member.to_string()); 
 			self.quick_replies.push(quick_reply);
 		}
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
		data["quick_replies"] = replies.into();
		data
	}
	
	fn can_perform_action_on_empty_input (&self) -> bool {
		false
	}

	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName) {
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

















#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct ButtonBlock {
	id : String,
	component_type : String,
	node_name : String,
	next_block_info : NextBlockInfo,
	delay : u16,
	
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
			text : String::new(),
			buttons : Vec::<ButtonImpl>::new()
		}
	}

	pub fn static_block() -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = "buttons".into();
		data["text"] = "Here are some articles that you requested.".into();
		data["delay"] = 500.into();
		let mut buttons = json::JsonValue::new_array();
		buttons.push(ButtonImpl::static_block());
		data["buttons"] = buttons.into(); 
		data
	}
}

impl Block for ButtonBlock {
	
	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();		
		assert_eq!(parsed["component_type"], "button"); 
		
		self.id 			= parsed["id"].to_string();
		self.component_type = parsed["component_type"].to_string();
		self.node_name 		= parsed["nodeName"].to_string();
		self.next_block_info.from_json(&parsed["metadata"].to_string());
		self.text 			= parsed["text"].to_string();
		self.delay 			= parsed["delay"].as_u16().unwrap();
		
		for member in parsed["buttons"].members() {
			let mut button = ButtonImpl::new();
			button.from_json(&member.to_string()); 
 			self.buttons.push(button);
 		}
	}

	fn convert_to_json(&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		let mut buttons = json::JsonValue::new_array();
		for button in self.buttons.iter() {
			buttons.push(button.convert_to_json());
		}

		data["component_type"] = self.component_type.clone().into();
		data["text"] = self.text.clone().into();
		data["delay"] = self.delay.into();
		data["buttons"] = buttons.into();
		data
	}
	
	fn can_perform_action_on_empty_input(&self) -> bool {	
		self.next_block_info.can_perform_action_on_empty_input()
	}

	fn perform_action(&self, user_input : &String, intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName) {
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











// "component_type": "jump",
//             "nodeName": "Jump Block 1",
//             "selectedResult": {
//               "value": "254c74b0-006d-11ec-b5a7-737ac2dca7c8",
//               "label": "Help Question",
//               "jumpType": "node"
//             },
//             "reprompt": [],
//             "metadata": [],
//             "selectedProject": "",
//             "id": "a99404a2-70ac-48c4-beba-339cc702ae0d"
 
//   "component_type": "jump",
//             "nodeName": "Jump to project..",
//             "selectedResult": {
//               "value": "49ec5fee-89a0-4420-b92a-173a3c8a3552",
//               "label": "Grant Question",
//               "jumpType": "node"
//             },
//             "reprompt": [],
//             "metadata": [],
//             "selectedProject": "",
//             "id": "a82db8de-1361-41df-985c-ed852e9de778"

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct JumpBlock {
	id : String,
	component_type : String,
	node_name : String,
	jump_to_nodename : String,
	jump_to_id	  : String
}

impl JumpBlock {
	pub fn new() -> Self {
		JumpBlock { 
			id : String::new(), 
			component_type : String::new(),
			node_name : String::new(),
			jump_to_nodename : String::new(),
			jump_to_id	  : String::new()
		}
	}
}

impl Block for JumpBlock {
	
	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();		
		assert_eq!(parsed["component_type"], "jump"); 
		
		// println!("Jump block: {:#?}", parsed["selectedResult"]);

		self.id 			= parsed["id"].to_string();
		self.component_type = parsed["component_type"].to_string();
		self.node_name 		= parsed["nodeName"].to_string();
		
		let mut iter = parsed["selectedResult"].entries();
		self.jump_to_id = iter.next().unwrap().1.to_string(); 
		self.jump_to_nodename = iter.next().unwrap().1.to_string(); 
		// println!("current nodename : {}", self.node);
		// println!("Jump to id : {}", jump_to_id);
		// println!("Jump to nodename : {}", jump_to_nodename);
		// println!("Jump block: {:#?}", self);
	}

	fn convert_to_json(&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = self.component_type.clone().into();
		data
	}
	
	fn can_perform_action_on_empty_input (&self) -> bool {
		true
	}

	fn perform_action(&self, _user_input : &String, _intents: &RefCell<HashMap<String, Box<dyn Intent>>>) -> (LinkType, IntentName, BlockName) {
		println!("Jumpblock performing action {}, {}", self.node_name, self.jump_to_nodename);
		(LinkType::Jump, String::new(), self.jump_to_nodename.clone())
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

