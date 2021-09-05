use std::collections::HashMap;

pub trait Button {
	fn from_json(&mut self, json_text : &str);
}
							
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ButtonImpl {
	button_type : String,
	title : String,
	payload : String
}

impl ButtonImpl {
	pub fn new() -> Self {
		ButtonImpl {
			button_type : String::new(),
			title : String::new(), 
			payload : String::new()
		}
	}

	pub fn static_block() -> json::JsonValue {
		let mut data = json::JsonValue::new_object();

		data["type"] = "postback".into();
		data["title"] = "What is DFINITY?".into();
		data["payload"] = "DFINITY built the Internet Computer—a new technology stack that reinvents the internet as a computer that hosts secure software and dapps.".into();
		data
	}
}

// impl Button for ButtonImpl {

// 	fn from_json(&mut self, json_text : &str) {
// 		let parsed = json::parse(json_text).unwrap();
// 	}
// }