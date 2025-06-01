mod scene;
mod cue;
mod sequence;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::definition::function::scene::Scene;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum Function {
    Scene(Scene),
    
}