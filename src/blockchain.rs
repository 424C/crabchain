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

        let mut hash_hex = hex::encode(hash.finalize());

        // mine the block until the hash starts with the target number of leading zeros (difficulty)
        // this is the proof of work and highly dependent on the difficulty.
        while !hash_hex.starts_with(&target) {
            nonce += 1;
            println!("Hash #{}: {}", nonce, hash_hex);
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

    // check if the chain is valid by comparing each block's previous hash to the hash of the previous block, they should all start with the same number of leading zeros
    // and link to each other
    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
    
        
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
    

            // also check the leading zeroes of the hash match the difficulty
            if !current_block.hash.starts_with(&previous_block.hash[0..self.difficulty as usize]) {
                return false;
            }
        }
    
        true
    }
}