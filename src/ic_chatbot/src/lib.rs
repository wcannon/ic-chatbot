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

use crate::store::{*}; 
use crate::block::{*};
// pub type JsonText = String; 


/*Returns a Json string of the form 
	{
		"component_type" : "session_id"
		"session_id" : "32134287198341"
	}
*/
#[update]
fn init_session () -> JsonText {
	let session = Session::new(); 
	session.convert_to_json()
}



/*
Returns a Json string corresponding to a vector blocks. 
[
	{
		"component_type" : "text",
		"text"			 : "These articles might be of help",
		"delay"			 : "500"
		"end_conversation" : false,
		"alternate_replies" : ["Here are some nice articles", 
								"You would love to read this"]
	},
	{
		"component_type" : "quick_replies",
		"text"			 : "These articles might be of help",
		"delay"			 : "500"
		"end_conversation" : false,
		"quick_replies"	 : [
								{
					                "content_type": "text",
					                "title": "Yes",
					                "image_url": "",
					                "payload": "Some"
					            },
					            {
					                "content_type": "text",
					                "title": "No",
					                "image_url": "",
					                "payload": "Thing"
					            }
						   ]
	},
	{
		"component_type" : "buttons",
		"text"			 : "These articles might be of help",
		"delay"			 : "500"
		"end_conversation" : false,
		"buttons"		 : [ { 
								"type" : "postback",
								"title" : "What is DFINITY?",
								"payload" : "DFINITY built the Internet Computer—a new technology stack that reinvents the internet as a computer that hosts secure software and dapps."
							},
							{
								"type" : "postback",
								"title" : "What is the DFINITY Foundation?",
								"payload" : "The DFINITY Foundation is a not-for-profit scientific research organization based in Zurich, Switzerland, that oversees research centers in Palo Alto, San Francisco, and Zurich, as well as teams in Japan, Germany, the UK, and across the United States."
							}
						  ]
	}
]
*/
#[update]
fn get_next_block (id : store::SessionId, user_input : String) -> JsonText {
	TextBlock::static_block()
}