pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, timestamp: u64, data: String, previous_hash: String) -> Block {
        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: u64, data: &str, previous_hash: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(index.to_string().as_bytes());
        hasher.update(timestamp.to_string().as_bytes());
        hasher.update(data.as_bytes());
        hasher.update(previous_hash.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}