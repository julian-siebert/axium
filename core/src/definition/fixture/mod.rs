pub mod model;
pub mod mode;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::id::{Identifier, Target};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Fixture {
    identity: Identifier,
    model: Target,
    mode: Target,
    universe: Target,
    
    address: u16,
    channels: Option<u16>,
    quantity: u16,
    address_gap: u16,
}