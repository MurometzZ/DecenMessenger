use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Chat {
        name: String,
        message: String
    },
    Join {
        name: String
    },
    Leave {
        name: String
    },
    Ping,
}
