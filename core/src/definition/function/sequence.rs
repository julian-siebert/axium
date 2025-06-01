use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::id::{Identifier, Target};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Sequence {
    identity: Identifier,
    cues: Vec<(u32, Target)>,
    
}