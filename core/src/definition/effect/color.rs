use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::id::{Identifier, Target};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct ColorEffect {
    identity: Identifier,
    fixtures: Vec<Target>,

    mode: ColorEffectMode,
    speed_hz: f32,
    color_palette: Vec<Target>
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Color {
    identity: Identifier,
    red: u16,
    green: u16,
    blue: u16,
    cyan: u16,
    magenta: u16,
    yellow: u16,
    amber: u16,
    white: u16,
    uv: u16,
    lime: u16,
    indigo: u16
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum ColorEffectMode {
    GradientLoop,
    Chase,
    Pulse,
}