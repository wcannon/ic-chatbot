extern crate natural;
// use natural::distance::jaro_winkler_distance;
// use natural::distance::levenshtein_distance;
use edit_distance::edit_distance;

pub trait TrainingPhrase : TrainingPhraseClone {
	fn from_json(&mut self, json_text : &str); 
	fn get_matching_score(&self, user_input : &str) -> usize; 
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

	fn get_matching_score (&self, user_input : &str) -> usize {
		println!("Matching score: {}, {}",self.text, user_input);
		println!("{}", edit_distance(&self.text.to_lowercase().trim(), &user_input.to_lowercase().trim()));
		edit_distance(&self.text.to_lowercase().trim(), &user_input.to_lowercase().trim())
	}
}