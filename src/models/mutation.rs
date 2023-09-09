use serde::{Serialize, Deserialize};
use mina_hasher::{Hashable, ROInput};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mutation {
    pub public_key: String,
    pub balance: u32,
}

impl Hashable for Mutation {
    type D = u32; 

    fn to_roinput(&self) -> ROInput {
        let pubkey_bytes = self.public_key.as_bytes();
        ROInput::new()
            .append_bytes(pubkey_bytes)
            .append_u32(self.balance as u32)
    }

    fn domain_string(seed: u32) -> Option<String> {
        Some(format!("MutationBalance {}", seed))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureJSON {
    pub r: String,
    pub s: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedMutation {
    pub mutation: Mutation,
    pub signature: SignatureJSON,
}