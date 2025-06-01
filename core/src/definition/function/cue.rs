use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::definition::delay::Delay;
use crate::id::{Identifier, Target};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Cue {
    identity: Identifier,
    scenes: Vec<Target>,
    delay: Delay,
}