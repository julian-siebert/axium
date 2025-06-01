use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Identifier {
    target: Target,
    name: String,
    description: Option<String>,
    labels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Target {
    group: u32,
    id: u32
}