
pub trait QuickReply {
	fn from_json(&mut self, json_text : &str);
	fn convert_to_json (&self) -> json::JsonValue; 

	//Returns true if the user input match exactly
	fn get_matching_score(&self, user_input : &str) -> bool; 
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct QuickReplyImpl {
	content_type : String,
	title : String, 
	image_url : String,
	payload : String
}

impl QuickReplyImpl {
	pub fn new() -> Self {
		QuickReplyImpl {
			content_type : String::new(),
			title : String::new(), 
			image_url : String::new(),
			payload : String::new()
		}
	}

	pub fn static_block(title : String) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["content_type"] = "text".into();
		data["title"] = title.into();
		data["image_url"] = "https://google.com".into();
		data["payload"] = "Some payload".into();
		data
	}
}

impl QuickReply for QuickReplyImpl {

	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();

		self.content_type 	= parsed["content_type"].to_string();
		self.title 			= parsed["title"].to_string();
		self.image_url 		= parsed["image_url"].to_string();
		self.payload 		= parsed["payload"].to_string();
	}

	fn convert_to_json(&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();
		data["content_type"] = self.content_type.clone().into();
		data["title"] 		 = self.title.clone().into();
		data["image_url"]	 = self.image_url.clone().into();
		data["payload"]		 = self.payload.clone().into();
		data
	}

	fn get_matching_score(&self, user_input : &str) -> bool {
		self.content_type == user_input
	}
}








// #[derive(PartialEq, Eq, Clone, Debug, Default)]
// pub struct QuickReplyInfo {
// 	quick_replies : Vec<Box<dyn QuickReply>>
// }

// impl QuickReplyInfo {
// 	pub fn new() -> Self {
// 		QuickReplyInfo {
// 			quick_replies : Vec::new()
// 		}
// 	}
// }

// impl for QuickReplyInfo {

// 	fn from_json(&mut self, json_text : &str) {
// 		let parsed = json::parse(json_text).unwrap();
// 		println!("{:#?}", parsed);
// 		for quick_reply in parsed.members() {
// 			self.quick_replies.push(quick_reply.to_string());
// 		}
// 	}

// 	fn convert_to_json(&self) -> json::JsonValue {
// 		let mut result = json::JsonValue::new_array();
// 		for quick_reply in quick_replies {
// 			result.push(quick_reply.convert_to_json());
// 		}
// 		result
// 	}
// }


