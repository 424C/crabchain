mod block;
mod blockchain;

use std::process;
use std::io;
use block::Block;
use blockchain::Blockchain;

fn display_menu() {
    println!();
    println!("**********************************");
    println!("CRABCHAIN MENU:");
    println!("1. Mine block");
    println!("2. Set difficulty");
    println!("3. View blockchain");
    println!("4. Validate blockchain");
    println!("5. Exit");
    println!("**********************************");
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
    // adjust the default difficulty for your hardware.
    // example: an apple silicon M1 (2020) takes on average about 600k - 700k hashes to hit a block which is a block time of around 7 seconds.
    // better hardware will require you to increase the difficulty.
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
                println!("Validating blockchain...");
                println!("Blockchain is {}", if blockchain.is_valid_chain() { "valid" } else { "invalid" });
                if !blockchain.is_valid_chain() {
                    println!("An error state has occurred.");
                    process::exit(1);
                }
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}