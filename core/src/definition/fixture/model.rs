use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::id::{Identifier, Target};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Model {
    identity: Identifier,
    manufacturer: String,
    model: String,
    model_type: ModelType,
    modes: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum ModelType {
    ColorChanger,
    Dimmer,
    Effect,
    Fan,
    Flower,
    Hazer,
    Laser,
    MovingHead,
    Other,
    Scanner,
    Smoke,
    Strobe,
    LEDBarBeams,
    LEDBarPixels
}
