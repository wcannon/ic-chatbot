use crate::trainingphrase::TrainingPhrase;
use crate::trainingphrase::TrainingPhraseImpl;


pub trait Intent : IntentClone {

	fn from_json(&mut self, intent_json_text : &str, training_phrase_json_text: &str);

	fn get_matching_score(&self, user_input : &str) -> usize;
	
	fn get_intent_name(&self) -> &str;
}

pub trait IntentClone {
	fn clone_box(&self) -> Box<dyn Intent>;
}

impl <T> IntentClone for T
where
	T : 'static + Intent + Clone,
{
	fn clone_box(&self) -> Box <dyn Intent> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn Intent> {
	fn clone(&self) -> Box<dyn Intent> {
		self.clone_box()
	}
}

#[derive(Clone, Default)]
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

	fn get_matching_score(&self, user_input : &str) -> usize {
		*self.training_phrases.iter().map(|phrase| {phrase.get_matching_score(user_input)}).collect::<Vec<usize>>().iter().min().unwrap()
	}

	fn get_intent_name(&self) -> &str {
		&self.intent_name
	}
}
