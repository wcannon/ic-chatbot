pub trait TrainingPhrase : TrainingPhraseClone {
	fn from_json(&mut self, json_text : &str); 
}

pub trait TrainingPhraseClone {
	fn clone_box(&self) -> Box<dyn TrainingPhrase>; 
}

impl<T> TrainingPhraseClone for T 
where 
	T : 'static + TrainingPhrase + Clone,
{
	fn clone_box(&self) -> Box<dyn TrainingPhrase> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn TrainingPhrase> {
	fn clone(&self) -> Box<dyn TrainingPhrase> {
		self.clone_box()
	}
}


#[derive(Clone, Default)]
pub struct TrainingPhraseImpl {
	id : String, 
	text : String
}

impl TrainingPhraseImpl { 
	pub fn new() -> Self {
		TrainingPhraseImpl {
			id : String::new(),
			text : String::new()
		}
	}
}

impl TrainingPhrase for TrainingPhraseImpl {

	fn from_json(&mut self, json_text : &str) {
		let parsed = json::parse(json_text).unwrap(); 

		self.id = parsed["id"].to_string();
		self.text = parsed["data"][0]["text"].to_string();

		println!("{:#?} {:#?}", self.id, self.text);
	}

	// fn match_user_input_with_training_phrase(user_input : &str) -> f64 {
		
	// }
}