mod block;
mod blockchain;

use std::io;
use block::Block;
use blockchain::Blockchain;

fn display_menu() {
    println!("Crabchain Menu:");
    println!("1. Mine block");
    println!("2. Set difficulty");
    println!("3. View blockchain");
    println!("4. Exit");
    println!("Enter your choice: ");
}

fn display_blockchain(blockchain: &Blockchain) {
    println!("Crabchain:");
    for block in &blockchain.chain {
        println!("Index: {}", block.index);
        println!("Timestamp: {}", block.timestamp);
        println!("Data: {}", block.data);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("-------------------------");
    }
}

fn main() {
    let mut difficulty = 5;
    let mut blockchain = Blockchain::new(difficulty);

    loop {
        display_menu();
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let new_block = blockchain.mine_block("".to_string());
                blockchain.add_block(new_block.unwrap());
                println!("New block mined and added to the blockchain!");
            }
            2 => {
                println!("Enter the new difficulty level:");
                let mut new_difficulty = String::new();
                io::stdin()
                    .read_line(&mut new_difficulty)
                    .expect("Failed to read input");

                let new_difficulty: u32 = match new_difficulty.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                blockchain.set_difficulty(new_difficulty);
                println!("Difficulty set to {}", new_difficulty);
            }
            3 => {
                display_blockchain(&blockchain);
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }

    for block in &blockchain.chain {
        println!("Index: {}", block.index);
        println!("Timestamp: {}", block.timestamp);
        println!("Data: {}", block.data);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("-------------------------");
    }

    }
}