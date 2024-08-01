# Crabchain

![Crabchain](assets/crabchain.png)
Welcome to Crabchain, a CLI blockchain implementation written in Rust.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

I built Crabchain as a way to familiarize myself with Rust. Crabchain doesn't have too many features, but it is pretty nimble and safe. It does allow for some of the most common blockchain operations such as mining, setting the difficulty level, and viewing the blockchain.

## Features

- Mine new blocks
  Mining blocks is the most common operation in blockchains. The block that is mined is added to the end of the chain, and the previous block's hash is used as the new block's previous hash. To mine a new block, use the `mine_block` command or by using the CLI.

- Set difficulty levels
  The difficulty level is the number of leading zeros in the hash of the block. The more leading zeros, the more difficult it is to mine a block. The default difficulty level is 5, which is tuned for most mid-range laptops. You can change the difficulty level using the `set_difficulty` command or by using the CLI.

- View the blockchain
  See the current state of the blockchain using the `view_blockchain` command or by using the CLI. Details included in the block are index, timestamp, data, previous hash, and hash.

## Installation

To install Crabchain, follow these steps:

1. Clone the repository: `git clone https://github.com/your-username/crabchain.git`
2. Navigate to the project directory: `cd crabchain`
3. Build the project: `cargo build --release`

## Usage

To use Crabchain, follow these steps:

1. Run the executable: `target/release/crabchain`
2. Select an option from the menu:
   - `1` - Mine a new block
   - `2` - Set difficulty level
   - `3` - View the blockchain
   - `4` - Exit

## Planned Features

- Add GUI

## Technologies Used

- Rust
- SHA256
- hex

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
