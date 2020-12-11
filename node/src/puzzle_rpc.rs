use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use sp_io::hashing::blake2_256;
//use codec::{Encode, Decode};

type Hash = [u8; 32];

#[rpc]
pub trait PuzzleRpc {
    #[rpc(name = "puzzle")]
    fn create_puzzle(&self, recipient: String, password: String) -> Result<Hash>;
}

pub struct Puzzle;

impl PuzzleRpc for Puzzle
{
    fn create_puzzle(&self, recipient: String, password: String) -> Result<Hash> {

        let mut data = Vec::from(recipient);
        data.append(&mut Vec::from(password));

        Ok(blake2_256(&data))
    }
}