/* 

This is the main file of the project. 
This file provides the external API that can be used by the frontend. 

*/
use std::io;
use ic_cdk_macros::update;

pub mod block;
pub mod metadata; 
pub mod factory;
pub mod intent;
pub mod trainingphrase;
pub mod store; 
mod tests;

pub type JsonText = String; 


/*Returns a Json string of the form 
	{
		"component_type" : "session_id"
		"session_id" : "32134287198341"
	}
*/
#[update]
fn init_session () -> JsonText {
	String::new()
}

#[update]
fn get_next_block (id : store::SessionId, user_input : String) -> JsonText {
	String::new()
}