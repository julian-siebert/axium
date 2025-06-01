use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::id::Identifier;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Universe {
    identity: Identifier,
    
}