use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::id::{Identifier, Target};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct MovementEffect {
    identity: Identifier,
    fixtures: Vec<Target>,
    
    waveform: Waveform,
    phase_offset: f32,
    speed_hz: f32,
    amplitude: u8,
    center: u8
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum Waveform {

}