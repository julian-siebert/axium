mod color;
mod movement;
mod audio;

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::{
    definition::effect::color::ColorEffect,
    definition::effect::movement::MovementEffect,
    definition::effect::audio::AudioEffect
};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum Effect {
    Color(ColorEffect),
    Movement(MovementEffect),
    Audio(AudioEffect),
}