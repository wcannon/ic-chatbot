
pub trait QuickReply {
	fn from_json(&mut self, json_text : &str);
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

// impl QuickReply for QuickReplyImpl {

// 	fn from_json(&mut self, json_text : &str) {
// 		let parsed = json::parse(json_text).unwrap();
// 	}
// }