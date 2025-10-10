use crate::{
    Wallet,
    core::{get_pubkey_preview, parse_args, to_base58},
};
use solana_sdk::signature::{Keypair, Signer};

pub fn new_command(wallets: &mut Vec<Wallet>) {
    println!("> Generating new...");

    let kp = Keypair::new();

    let new_wallet = Wallet {
        pubkey: kp.pubkey().to_string(),
        pubkey_bytes: kp.pubkey().to_bytes(),
        secret_key: kp.secret_bytes().clone(),
        tag: get_pubkey_preview(&kp.pubkey().to_string()),
        was_grinded: false,
        grind_job_id: String::from(""),
    };

    println!(
        "> Generated new wallet {} with tag {}",
        new_wallet.pubkey, new_wallet.tag
    );

    for wallet in wallets.iter() {
        if wallet.tag == new_wallet.tag {
            println!("> ERROR: Tag already exists");
            return;
        }
    }

    wallets.push(new_wallet);
}

pub fn list_command(wallets: &Vec<Wallet>) {
    if wallets.len() == 0 {
        println!("> No wallets");
        return;
    }

    for wallet in wallets.iter() {
        println!("> {}: {}", wallet.tag, wallet.pubkey)
    }
}

pub fn secret_key_command(wallets: &Vec<Wallet>, args: &str) {
    let wallet_input = args.trim();
    let mut wallet: Option<&Wallet> = None;

    for w in wallets.iter() {
        if w.tag == wallet_input || w.pubkey == wallet_input {
            wallet = Some(w);
            break;
        }
    }

    let mut full_secret: [u8; 64] = [0; 64];

    for (i, byte) in wallet.unwrap().secret_key.iter().enumerate() {
        full_secret[i] = *byte;
    }
    for (i, byte) in wallet.unwrap().pubkey_bytes.iter().enumerate() {
        full_secret[i + 32] = *byte;
    }

    if wallet.is_none() {
        println!("Could not find wallet that matches {}", wallet_input)
    } else {
        println!("{:?}\n{}", full_secret, to_base58(&full_secret))
    }
}

pub fn grind_command(wallets: &mut Vec<Wallet>, args: &str) {
    let arguments = parse_args(args);

    let mut start_condition = "";
    let mut end_condition = "";
    let mut count = 1u32;

    for arg in &arguments {
        match arg.arg_name.as_str() {
            "start" => start_condition = arg.parameters[0].as_str(),
            "end" => end_condition = arg.parameters[0].as_str(),
            "count" => count = arg.parameters[0].parse::<u32>().unwrap(),
            _ => {}
        }
    }

    if start_condition.is_empty() && end_condition.is_empty() {
        println!("> Need either a start or end condition to grind");
        return;
    }

    let mut found = 0u32;
    let mut tries = 0u128;

    while (found < count) {
        let kp = Keypair::new();
        let address = kp.pubkey().to_string();

        tries += 1;
        if tries % 10_000 == 0 {
            println!("Grinded {} addresses", tries)
        }

        if !start_condition.is_empty() && !address.starts_with(start_condition) {
            continue;
        } else if !end_condition.is_empty() && !address.ends_with(end_condition) {
            continue;
        }

        let new_wallet = Wallet {
            pubkey: kp.pubkey().to_string(),
            pubkey_bytes: kp.pubkey().to_bytes(),
            secret_key: kp.secret_bytes().clone(),
            tag: get_pubkey_preview(&kp.pubkey().to_string()),
            was_grinded: true,
            grind_job_id: String::from(""),
        };

        wallets.push(new_wallet);
        found += 1;

        println!("Found {} | {}/{}", address, found, count);
    }
}
