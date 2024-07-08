use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    name: String,
    #[serde(rename = "birthYear")]
    b_year: Option<u32>,
}

// Implementation block for type Person
impl Person {
    fn new(name: String, b_year: Option<u32>) -> Self {
        Self { name, b_year: b_year }
    }
}