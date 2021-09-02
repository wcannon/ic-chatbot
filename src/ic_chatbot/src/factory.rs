use std::fs;
use crate::intent::Intent;
use crate::intent::IntentImpl;


pub trait Factory {
	fn load_json_files(intent_directory : &str); 
}

pub struct FactoryImpl {

}

impl Factory for FactoryImpl {
	
	fn load_json_files(intent_directory : &str) {
		let mut intents = Vec::<Box<dyn Intent>>::new();

		let paths = fs::read_dir(intent_directory).unwrap();
		for path in paths {
			let path = path.unwrap().path(); 
			let path = path.to_str().unwrap();

			if !path.contains("usersays_en.json") && !path.contains("Default_Fallback_Intent") {
				let intent_path = path.clone();
				let mut training_phrase_path = path.trim_end_matches(".json").to_string(); 
				training_phrase_path.push_str("_usersays_en.json");
				
				let intent_json_text = fs::read_to_string(intent_path).expect("Something went wrong reading the intent file");
				let training_phrase_json_text = fs::read_to_string(training_phrase_path).expect("Something went wrong reading the training phrase file");

				let mut current_intent : IntentImpl = IntentImpl::new();
				current_intent.from_json(intent_json_text, training_phrase_json_text);  
				intents.push( Box::new(current_intent) ); 
				// println!("Name: {}", intent_path);
				// println!("Name: {}", training_phrase_path);	
			
			}

		}
	}
}