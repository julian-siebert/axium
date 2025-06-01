use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum Message {
    Ping {
        node: u32,
    },
    Pong {
        node: u32,
        
    },
    Pull {
        
    }
}