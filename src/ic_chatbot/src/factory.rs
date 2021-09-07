use std::fs;
use crate::intent::Intent;
use crate::intent::IntentImpl;
use crate::block::{*}; 

pub trait Factory {
	fn load_json_files(intent_directory : &str, blocks_file : &str) -> (Vec::<Box<dyn Block>>, Vec::<Box<dyn Intent>>); 
	fn load_blocks_json (blocks_json_text : String) -> Vec::<Box<dyn Block>>; 
}

pub struct FactoryImpl {

}

impl Factory for FactoryImpl {
	
	fn load_json_files(intent_directory : &str, blocks_file : &str) -> (Vec::<Box<dyn Block>>, Vec::<Box<dyn Intent>>) {
		let mut intents = Vec::<Box<dyn Intent>>::new();
		let mut blocks = Vec::<Box<dyn Block>>::new();

		//Loading all the intents and their training phrases into intents vector. 
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
				current_intent.from_json(intent_json_text.as_str(), training_phrase_json_text.as_str());  
				intents.push( Box::new(current_intent) ); 
			
			}
		}

		//Loading all the blocks into blocks vector. 
		let blocks_json_text = fs::read_to_string(blocks_file).expect("Something went wrong reading the blocks file");
		let parsed = json::parse(blocks_json_text.as_str()).unwrap();
		// println!("{:#?}", parsed); 
		for (id, member) in parsed.entries() {
			let member = &member["en"]["generic"]["blocks"][0];
			if member.has_key("component_type") {
				println!("{}, {:#?}", id, member);
				let mut block : Box::<dyn Block> = match member["component_type"].to_string().as_str() {
					"text" => Box::new(TextBlock::new()),
					"quick_replies" => Box::new(QuickRepliesBlock::new()),
					"jump" => Box::new(JumpBlock::new()),
					"button" => Box::new(ButtonBlock::new()),
					"start_block" => Box::new(StartBlock::new()),
					_ => Box::new(TextBlock::new())
				};
				block.from_json(member.to_string().as_str());
				blocks.push(block);
			}
		}
		
		(blocks, intents)
	}


	fn load_blocks_json (blocks_json_text : String) -> Vec::<Box<dyn Block>> {
		//Loading all the blocks into blocks vector. 
		let mut blocks = Vec::<Box<dyn Block>>::new();
		// let blocks_json_text = fs::read_to_string(blocks_file).expect("Something went wrong reading the intent file");
		let parsed = json::parse(blocks_json_text.as_str()).unwrap();
		// println!("{:#?}", parsed); 
		for (id, member) in parsed.entries() {
			let member = &member["en"]["generic"]["blocks"][0];
			if member.has_key("component_type") {
				println!("{}, {:#?}", id, member);
				let mut block = match member["component_type"].to_string().as_str() {
					"text" => TextBlock::new(),
					_ => TextBlock::new()
				};

				block.from_json(member.to_string().as_str());
				blocks.push(Box::new(block));
			}
		}
		blocks
	}
}
