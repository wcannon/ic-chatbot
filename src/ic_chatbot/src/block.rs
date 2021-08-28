
mod block {
	
	trait Block {

		fn load_from_json(&mut self, json_text : String);
	
		fn convert_to_json(&self) -> String;  
		
		fn perform_action(&mut self); 
	}


	struct TextBlock {

	}

	impl Block for TextBlock {

		fn load_from_json(&mut self, json_text : String) {

		}
	
		fn convert_to_json(&self) -> String {
			String::new()
		}
		
		fn perform_action(&mut self) {

		}

	}

}