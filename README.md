# Checkers.rs 

A rust implementation of Checkers in the terminal. Includes a bot to play against. 

## Requirements 

- Rust 1.72.0 or newer, available [here](https://www.rust-lang.org/tools/install) 
- A terminal using a [nerd font](https://www.nerdfonts.com/#home) 
- A terminal which supports truecolor, see [here](https://github.com/termstandard/colors#truecolor-support-in-output-devices) for a full list  

## Running the program 

- Clone the repo 

    `$ git clone https://github.com/Henry-Ash-Williams/checkers`
    
    `$ cd checkers`

- Verify rust and cargo are installed 

    `$ rustc --version`

    `rustc 1.73.0 (cc66ad468 2023-10-03)`

    `$ cargo version`

    `cargo 1.73.0 (9c4383fb5 2023-08-26)`

- Build the program 

    `$ cargo build --release`

- Run the program 

    `$ ./target/release/checkers`