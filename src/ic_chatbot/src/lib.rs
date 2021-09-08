/* 

This is the main file of the project. 
This file provides the external API that can be used by the frontend. 

*/
// use std::io;
use ic_cdk_macros::update;

pub mod block;
pub mod metadata; 
pub mod quickreply;
pub mod button;
pub mod factory;
pub mod intent;
pub mod trainingphrase;
pub mod store; 
pub mod types;
mod tests;

use crate::types::{*};
// use crate::block::{*};
use crate::store::{*}; 
use crate::factory::{*}; 

/*Returns a Json string corresponding to session id along with intial set of blocks.   
	{
		"component_type" : "session_id"
		"session_id" : "32134287198341"
	},
	{
		"component_type" : "text",
		"text"			 : "Welcome to Dfinity.",
		"delay"			 : "500"
		"end_conversation" : false,
		"alternate_replies" : ["Hello, there!", 
								"How can we help you?"]
	}
*/
#[update]
fn init_session () -> JsonText {
	let session = Session::new();
	let mut result = json::JsonValue::new_array();
	result.push(session.convert_to_json());
	// for block in session.process_user_input(String::new()).members() {
	// 	result.push(block);
	// }
	result.push(Session::process_user_input(session.get_session_id().to_string(), String::new()));
	result.dump()
}



/*
Returns a Json string corresponding to a vector of blocks. 
[
	{
		"component_type" : "text",
		"text"			 : "Internet Computer is Cloud 3.0.",
		"delay"			 : "500"
		"end_conversation" : false,
		"alternate_replies" : ["Internet Computer is a next generation cloud service.", 
								"Internet Computer is developed by Dfinity foundation."]
	},
	{
		"component_type" : "buttons",
		"text"			 : "These articles might be of help",
		"delay"			 : "500"
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
	{
		"component_type" : "quick_replies",
		"text"			 : "Does this answer your question?",
		"delay"			 : "500"
		"quick_replies"	 : [
								{
					                "content_type": "text",
					                "title": "Yes",
					                "image_url": "",
					                "payload": "Some payload"
					            },
					            {
					                "content_type": "text",
					                "title": "No",
					                "image_url": "",
					                "payload": "Some payload"
					            }
						   ]
	},
]
*/
#[update]
fn get_next_block (session_id : store::SessionId, user_input : String) -> JsonText {
	Session::process_user_input(session_id, user_input).dump()

	// let mut result = json::JsonValue::new_array();
	// result.push(TextBlock::static_block());
	// result.push(ButtonBlock::static_block());
	// result.push(QuickRepliesBlock::static_block());
	// // result.push(TextBlock::static_block());
	// result.dump()
}

// #[update]
// fn get_next_block_temp (id : store::SessionId, user_input : String) -> JsonText {
// 	let mut result = json::JsonValue::new_array();
// 	result.push(TextBlock::static_block());
// 	result.push(QuickRepliesBlock::static_block());
// 	result.push(ButtonBlock::static_block());
// 	result.dump()
// }

#[update]
fn load_blocks_from_json (blocks_json_text : JsonText) {
	let blocks = FactoryImpl::load_blocks_json(blocks_json_text);
	store_blocks_in_state(blocks);
}

#[update]
fn load_intents_from_json (intents_json_text : JsonText) {
	let intents = FactoryImpl::load_intents_json(intents_json_text);
	store_intents_in_state(intents);
}

#[update]
fn get_all_blocks () -> JsonText {
	summarize_all_blocks().dump() 
}


