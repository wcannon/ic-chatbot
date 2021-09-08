pub type NodeName = String;
pub type IntentName = String;
pub type UserInput = String;
pub type JsonText = String; 
pub type SessionId = String;
pub type BlockName = String;
pub type InputName = String;

#[derive(PartialEq)]
pub enum LinkType {
	default,
	intent,
	response,
	jump,
	endofchart,
	wronginput
}

impl LinkType {
	pub fn to_str(&self) -> &str {
		match self {
			default => "default",
			intent => "intent",
			response => "response",
			jump => "jump",
			endofchart => "endofchart",
			wronginput => "wronginput"
		}
	}
}