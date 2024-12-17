use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HashingData {
    pub input: Vec<u8>,
    pub output: [u8; 32],
}