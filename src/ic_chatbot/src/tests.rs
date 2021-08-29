
#[cfg(test)]
mod tests {


    #[test]
    fn test_load_from_json() {
        pub use crate::block::Block;

        let blk = crate::block::TextBlock::from_json(r#"{ "component_type": "text",
            "nodeName": "Bot Says",
            "context": [],
            "text": "Welcome to Dfinity! We have a surprise for you. \nWhat's your sweet name by the way? ",
            "alternate_replies": [
              "Hello buddy, we have something exciting waiting for you. Let's first get to know each other. My name's blockchain. What's your name? "
            ],
            "audio_file": "",
            "ssml": "",
            "delay": 500,
            "reprompt": [],
            "metadata": [],
            "end_conversation": false,
            "buttons": [],
            "id": "61cecdb1-5af8-4747-8cb4-0a94950cd24f"} "#); 
        assert_eq!(String::from("61cecdb1-5af8-4747-8cb4-0a94950cd24f"), blk.id);
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
