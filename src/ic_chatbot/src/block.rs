pub trait Block {

	fn from_json(json_text : &str) -> Self;

	fn load_from_json(&mut self, json_text : &str);

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
	node_name : String,
	// metadata : Box<metadata>,
	delay : u16,
	end_conversation : bool,

	text : String,
	// alternate_replies : Vec<String>
}

impl Block for TextBlock {


	fn from_json(json_text : &str) -> Self {
		let parsed = json::parse(json_text).unwrap();
		
		TextBlock {
			id : parsed["id"].to_string(),
			component_type : parsed["component_type"].to_string(),			
			node_name : parsed["nodeName"].to_string(),
			text : parsed["text"].to_string(),
			delay : parsed["delay"].as_u16().unwrap(),
			end_conversation : parsed["end_conversation"].as_bool().unwrap()	
		}
	}

	fn load_from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap();
		
		self.id = parsed["id"].to_string();
		self.component_type = parsed["component_type"].to_string(); 
		self.node_name = parsed["nodeName"].to_string();
		self.text = parsed["text"].to_string();
		// self.alternate_replies = parsed["alternate_replies"];
		self.delay = parsed["delay"].as_u16().unwrap();
		self.end_conversation = parsed["end_conversation"].as_bool().unwrap();
	}

	fn convert_to_json(&self) -> String {
		String::new()
	}
	
	fn perform_action(&mut self) {

	}

	// //Getter methods
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