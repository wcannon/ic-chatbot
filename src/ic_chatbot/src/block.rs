pub use crate::metadata::NextBlockInfo;
pub use crate::metadata::MetaData;

pub trait Block {

	fn from_json(&mut self, json_text : &str);

	fn convert_to_json(&self) -> String;
	
	fn perform_action(&mut self);


	//Getter Methods 
	fn get_id(&self) -> &str; 

	fn get_component_type(&self) -> &str; 

	fn get_node_name(&self) -> &str; 

	// fn get_delay(&self) -> &u16; 

	// fn get_end_conversation(&self) -> &bool; 
}





#[derive(PartialEq, Eq, Debug, Default)]
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

	// fn from_json(json_text : &str) -> Self {
	// 	let parsed = json::parse(json_text).unwrap();		
	// 	assert_eq!(parsed["component_type"], "text"); 

	// 	let mut alternate_replies : Vec<String> = Vec::<String>::new();
	// 	for alternate_reply in parsed["alternate_replies"].members() {
	// 		alternate_replies.push(alternate_reply.to_string());
	// 	}

	// 	TextBlock {
	// 		id : parsed["id"].to_string(),
	// 		component_type : parsed["component_type"].to_string(),
	// 		node_name : parsed["nodeName"].to_string(),
	// 		next_block_info : crate::metadata::NextBlockInfo::from_json(&parsed["metadata"].to_string()),
	// 		text : parsed["text"].to_string(),
	// 		delay : parsed["delay"].as_u16().unwrap(),
	// 		end_conversation : parsed["end_conversation"].as_bool().unwrap(),
	// 		alternate_replies : alternate_replies
	// 	}
	// }

	fn convert_to_json(&self) -> String {
		let mut data = json::JsonValue::new_object();

		data["component_type"] = self.component_type.clone().into();
		data["text"] = self.text.clone().into();
		data["delay"] = self.delay.into();
		data["end_conversation"] = self.end_conversation.into();
		data["alternate_replies"] = self.alternate_replies.clone().into();
		// assert_eq!(data.dump(), r#"{"answer":42,"foo":"bar"}"#);
		data.dump()
	}
	
	fn perform_action(&mut self) {

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







#[derive(PartialEq, Eq, Debug)]
pub struct QuickRepliesBlock {
	pub id : String,
	pub component_type : String,
	node_name : String,
	// metadata : Box<metadata>,
	delay : u16,
	end_conversation : bool,

	text : String,
	// alternate_replies : Vec<String>
}
