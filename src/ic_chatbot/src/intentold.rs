use crate::trainingphrase::TrainingPhrase;
use crate::trainingphrase::TrainingPhraseImpl;

pub trait Intent {

	fn from_json(&mut self, intent_json_text : &str, training_phrase_json_text: &str);

	fn match_user_input_with_intent(&self, user_input : &str) -> (String, f64); 	//Outputs training phrase with 

	// fn convert_to_json(&self) -> String;
	
	// fn perform_action(&mut self);

	fn get_intent_name(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct IntentImpl {
	id : String,
	intent_name : String, 
	training_phrases : Vec::<Box<dyn TrainingPhrase>>
}

impl IntentImpl {
	pub fn new() -> Self {
		IntentImpl {
			id : String::new(),
			intent_name : String::new(),
			training_phrases : Vec::new()
		}
	}
}

impl Intent for IntentImpl {
	
	fn from_json(&mut self, intent_json_text : &str, training_phrase_json_text: &str) {
		
		//Loading the id and name of the intent
		let parsed = json::parse(intent_json_text).unwrap();
		self.id = parsed["id"].to_string();
		self.intent_name = parsed["responses"][0]["action"].to_string();
		println!("Processing Intent : {:#?}", self.intent_name); 
		
		//Loading all the training phrases
		let parsed = json::parse(training_phrase_json_text).unwrap();
		for member in parsed.members() {
			let mut training_phrase = TrainingPhraseImpl::new(); 
			training_phrase.from_json(member.to_string().as_str());
			self.training_phrases.push(Box::new(training_phrase));  
		}
	}

	fn match_user_input_with_intent(&self, user_input : &str) -> (String, f64) {
		(String::new(), 0.0)
	}

	fn get_intent_name(&self) -> &str {
		&self.intent_name
	}
}
