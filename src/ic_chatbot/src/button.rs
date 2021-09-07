
pub trait Button {
	fn from_json(&mut self, json_text : &str);
	fn convert_to_json(&self) -> json::JsonValue;
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

impl Button for ButtonImpl {

	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();

		self.button_type 	= parsed["type"].to_string();
		self.title 			= parsed["title"].to_string();
		self.payload 		= parsed["payload"].to_string();
	}

	fn convert_to_json(&self) -> json::JsonValue {
		let mut data = json::JsonValue::new_object();
		data["type"] 		 = self.button_type.clone().into();
		data["title"] 		 = self.title.clone().into();
		data["payload"]		 = self.payload.clone().into();
		data
	}
}
