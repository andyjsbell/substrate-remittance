use jsonrpc_core::{Result, Error};
use jsonrpc_derive::rpc;
use sp_core::blake2_256;

type Hash = [u8; 32];

#[rpc]
pub trait PuzzleRpc {
    #[rpc(name = "puzzle")]
    fn create_puzzle(&self, recipient: String, password: String) -> Result<String>;
}

pub struct Puzzle;

impl PuzzleRpc for Puzzle
{
    fn create_puzzle(&self, recipient: String, password: String) -> Result<String> {

        if recipient == "" || password == "" {
            return Err(Error::new(jsonrpc_core::types::error::ErrorCode::InvalidRequest))
        }
        let mut data = Vec::from(recipient);
        data.append(&mut Vec::from(password));

        let hash = blake2_256(&data);

        Ok(hex::encode(hash))
    }
}