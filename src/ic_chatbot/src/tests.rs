
#[cfg(test)]
mod tests {


    #[test]
    fn test_load_from_json() {
        pub use crate::block::Block;

        let blk = crate::block::TextBlock::from_json(r#"{
            "component_type": "text",
            "nodeName": "Help Question",
            "context": [
              {
                "id": "9e75b110-01c7-11ec-9825-31b611849613",
                "name": "%%username%",
                "type": "text",
                "entity": "botmock.name",
                "default_value": "buddy",
                "start_index": 4
              }
            ],
            "text": "Hey %%username%, how can we help you? ",
            "alternate_replies": ["Hello buddy, we have something exciting waiting for you. Let's first get to know each other. My name's blockchain. What's your name? ",
              "What's your name by the way? "],
            "audio_file": "",
            "ssml": "",
            "delay": 500,
            "reprompt": [],
            "metadata": [
              {
                "item_key": "Intent-Grants",
                "item_value": "Grant Question"
              },
              {
                "item_key": "Intent-IC",
                "item_value": "Common Question"
              }
            ],
            "end_conversation": false,
            "buttons": [],
            "id": "254c74b0-006d-11ec-b5a7-737ac2dca7c8"
          }"#);

        println!("Loaded the following json: "); 
        println!("{:#?}", blk); 
        assert_eq!(String::from("254c74b0-006d-11ec-b5a7-737ac2dca7c8"), blk.id);
        
        let json_response = blk.convert_to_json();
        println!("Converted to the following json: "); 
        println!("{:#?}", json_response); 

        assert_eq!(json_response, 
                r#"{"component_type":"text","text":"Hey %%username%, how can we help you? ","delay":500,"end_conversation":false,"alternate_replies":["Hello buddy, we have something exciting waiting for you. Let's first get to know each other. My name's blockchain. What's your name? ","What's your name by the way? "]}"#);
    }

    #[test]
    fn parse_json() {

        let parsed = json::parse(r#"

        {
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ]
            }
        }

        "#).unwrap();

        let instantiated = json::object!{
            // quotes on keys are optional
            "code": 200,
            success: true,
            payload: {
                features: [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ]
            }
        };
        // println!("parsed: {}", parsed); 
        // println!("instantiated: {}", instantiated); 
        assert_eq!(parsed, instantiated);
    }
}
