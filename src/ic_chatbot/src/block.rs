pub use crate::metadata::NextBlockInfo;
pub use crate::metadata::MetaData;

pub trait Block {

	fn from_json(json_text : &str) -> Self;

	fn convert_to_json(&self) -> String;
	
	fn perform_action(&mut self);


	//Getter Methods 
	// fn get_id(&self) -> &'static String; 

	// fn get_component_type(&self) -> &'static String; 

	// fn get_node_name(&self) -> &'static String; 

	// fn get_delay(&self) -> &'static u16; 

	// fn get_end_conversation(&self) -> &'static bool; 
}





#[derive(PartialEq, Eq, Debug)]
pub struct TextBlock {
	pub id : String,
	pub component_type : String,
	pub node_name : String,
	next_block_info : NextBlockInfo,
	delay : u16,
	end_conversation : bool,

	text : String,
	alternate_replies : Vec<String>
}

impl Block for TextBlock {

	fn from_json(json_text : &str) -> Self {
		let parsed = json::parse(json_text).unwrap();		
		assert_eq!(parsed["component_type"], "text"); 

		let mut alternate_replies : Vec<String> = Vec::<String>::new();
		for alternate_reply in parsed["alternate_replies"].members() {
			alternate_replies.push(alternate_reply.to_string());
		}

		TextBlock {
			id : parsed["id"].to_string(),
			component_type : parsed["component_type"].to_string(),
			node_name : parsed["nodeName"].to_string(),
			next_block_info : crate::metadata::NextBlockInfo::from_json(&parsed["metadata"].to_string()),
			text : parsed["text"].to_string(),
			delay : parsed["delay"].as_u16().unwrap(),
			end_conversation : parsed["end_conversation"].as_bool().unwrap(),
			alternate_replies : alternate_replies
		}
	}

	// fn load_from_json(&mut self, json_text : &str) {
	// 	let parsed = json::parse(json_text).unwrap();
		
	// 	self.id = parsed["id"].to_string();
	// 	self.component_type = parsed["component_type"].to_string();
	// 	self.node_name = parsed["nodeName"].to_string();
	// 	self.text = parsed["text"].to_string();
	// 	// self.alternate_replies = parsed["alternate_replies"];
	// 	self.delay = parsed["delay"].as_u16().unwrap();
	// 	self.end_conversation = parsed["end_conversation"].as_bool().unwrap();
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
	// fn get_id(&'static self) -> &'static String {
	// 	&self.id
	// }

	// fn get_component_type(&'static self) -> &'static String {
	// 	&self.component_type
	// } 

	// fn get_node_name(&'static self) -> &'static String {
	// 	&self.node_name
	// }

	// fn get_delay(&'static self) -> &'static u16 {
	// 	&self.delay
	// }

	// fn get_end_conversation(&'static self) ->  &'static bool {
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
