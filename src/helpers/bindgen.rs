use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FarmCharacter {
    name: String,
    description: String,
    image: String,
    spritesheet: String,
    attributes: Vec<Attribute>,
    token_id: String,
}

#[wasm_bindgen]
pub fn load_json_asset(json_data: JsValue) -> Result<JsValue, JsValue> {
    let result: Result<FarmCharacter, serde_wasm_bindgen::Error> =
        serde_wasm_bindgen::from_value(json_data);

    match result {
        Ok(character_data) => {
            // Do something with my_data
            info!("Character Data {:?}", character_data);
            // Return the processed data or a custom result as JsValue
            Ok(JsValue::from_str("Success"))
        }
        Err(e) => {
            // Return a custom error message as a JsValue
            Err(JsValue::from_str(&format!("Error: {:?}", e)))
        }
    }
}
