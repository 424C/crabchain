use std::vec::Vec;
use sha2::{Sha256, Digest};
use hex;
use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Blockchain {
        Blockchain { chain: Vec::new(), difficulty }
    }

    pub fn set_difficulty(&mut self, difficulty: u32) {
        self.difficulty = difficulty;
    }

    pub fn mine_block(&mut self, data: String) -> Result<Block, String> {
        let index = self.chain.len() as u32;
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let mut hash = Sha256::new();
        let mut nonce = 0;

        // leading zeros of the hash are the difficulty
        let target = "0".repeat(self.difficulty as usize);

        // get previous hash, but first check if the chain is empty
        let previous_hash = if self.chain.is_empty() {
            "0".to_string()
        } else {
            self.chain.last().unwrap().hash.clone()
        };

        let mut hash = Sha256::new();
        let mut hash_hex = hex::encode(hash.finalize());

        // mine the block until the hash starts with the target number of leading zeros (difficulty)
        while !hash_hex.starts_with(&target) {
            nonce += 1;
            println!("Hash: {}", hash_hex);
            hash = Sha256::new();
            hash.update(format!("{}{}{}{}", index, timestamp, previous_hash, data));
            hash.update(nonce.to_string().as_bytes());
            hash_hex = hex::encode(hash.finalize());
        }

        Ok(Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: hash_hex,
        })
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    /*

    pub fn add_block(&mut self, data: String) {
        let index = self.chain.len() as u32;
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let mut hash = Sha256::new();
        let mut nonce = 0;

        // leading zeros of the hash are the difficulty
        let target = "0".repeat(self.difficulty as usize);
        
        // get previous hash, but first check if the chain is empty
        let previous_hash = if self.chain.is_empty() {
            "0".to_string()
        } else {
            self.chain.last().unwrap().hash.clone()
        };

        let mut hash = Sha256::new();
        let mut hash_hex = hex::encode(hash.finalize());

        // mine the block until the hash starts with the target number of leading zeros (difficulty)
        while !hash_hex.starts_with(&target) {
            nonce += 1;
            println!("Hash: {}", hash_hex);
            hash = Sha256::new();
            hash.update(format!("{}{}{}{}", index, timestamp, previous_hash, data));
            hash.update(nonce.to_string().as_bytes());
            hash_hex = hex::encode(hash.finalize());
        }
        
        let block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: hash_hex,
        };
        self.chain.push(block);
    }
        */
}