use serde::{Deserialize, Serialize};
use struct_input::StructInput;

#[derive(Serialize, Deserialize, Clone, Debug, StructInput)]
pub struct ClientConfig {
    #[struct_input(format="Name")]
    pub name: String,
    #[struct_input(format="Name", message="--- kind (ex: telegram) ---")]
    pub kind: String, // ex: telegram
    #[struct_input(format="NotAllowWhitespace")]
    pub token: Option<String>
}

impl ClientConfig {
    pub fn new_telegram(name: &str, token: &str) -> Self {
        Self {
            name: String::from(name),
            kind: String::from("telegram"),
            token: Some(String::from(token))
        }
    }
}