use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub use crate::definition::{
    fixture::Fixture,
    function::Function,
    universe::Universe
};

mod universe;
mod fixture;
mod channel;
mod effect;
mod function;
mod delay;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum Definition {
    Universe(Universe),
    Fixture(Fixture),
    Function(Function),
}