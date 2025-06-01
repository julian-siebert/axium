use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::id::{Identifier, Target};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Scene {
    identity: Identifier,
    effects: Vec<Target>,
    static_values: Vec<StaticValue>
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct StaticValue {
    channel: Target,
    value: u8
}