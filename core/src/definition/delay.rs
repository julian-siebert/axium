use std::time::Duration;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Delay {
    fade_in: Option<Duration>,
    hold: Option<Duration>,
    fade_out: Option<Duration>
}