mod commands;
mod core;
mod wallet;
use commands::{list_command, new_command};
use std::io;
use wallet::Wallet;

use crate::commands::{grind_command, secret_key_command};

fn main() {
    let mut wallets: Vec<Wallet> = Vec::new();

    let mut input = String::new();

    while input != "exit\n" {
        input = String::from("");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let trimmed = input.trim();

        let split_command = trimmed.split_once(' ');
        let mut command_name = trimmed;
        let mut args = "";
        if split_command != None {
            (command_name, args) = split_command.unwrap();
        } else if trimmed.len() == 0 {
            println!("> No command detected\n");
            continue;
        }

        match command_name {
            "new" => new_command(&mut wallets),
            "list" => list_command(&wallets),
            "sk" => secret_key_command(&wallets, args),
            "grind" => grind_command(&mut wallets, args),
            "exit" => {}
            _ => println!("> Unkown command"),
        }

        println!()
    }
}
