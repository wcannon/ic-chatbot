pub type NodeName = String;
pub type IntentName = String;
pub type UserInput = String;
pub type JsonText = String; 
pub type SessionId = String;
pub type BlockName = String;
pub type InputName = String;

#[derive(PartialEq)]
pub enum LinkType {
	Defaultlink,
	Intent,
	Response,
	Jump,
	Endofchart,
	Wronginput
}

impl LinkType {
	pub fn to_str(&self) -> &str {
		match self {
			LinkType::Defaultlink => "default",
			LinkType::Intent => "intent",
			LinkType::Response => "response",
			LinkType::Jump => "jump",
			LinkType::Endofchart => "endofchart",
			LinkType::Wronginput => "wronginput"
		}
	}
}