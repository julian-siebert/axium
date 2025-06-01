use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::id::Identifier;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Channel {
    identity: Identifier,
    /// Channels responsibility
    parameter: Parameter,
    capability: Capability,
    /// Fine-role enabled?
    fine: bool,
    default_value: u8,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum Parameter {
    Beam,
    Color,
    Effect,
    Gobo,
    Intensity,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    Amber,
    White,
    UV,
    Lime,
    Indigo,
    Maintenance,
    Nothing,
    Pan,
    Prism,
    Shutter,
    Speed,
    Tilt
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Capability {
    min_value: u8,
    max_value: u8,
    description: Option<String>,
}