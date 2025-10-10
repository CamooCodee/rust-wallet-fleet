use bs58;

pub fn get_pubkey_preview(pubkey: &String) -> String {
    let mut preview: String = String::from("");
    let len = pubkey.len();
    let preview_start = &pubkey[..4];
    let preview_end = &pubkey[len - 4..];

    preview.push_str(preview_start);
    preview.push_str("...");
    preview.push_str(preview_end);

    return preview;
}

pub fn to_base58(secret: &[u8]) -> String {
    return bs58::encode(secret).into_string();
}

pub struct Argument {
    pub arg_name: String,
    pub parameters: Vec<String>,
}

pub fn parse_args(args: &str) -> Vec<Argument> {
    let mut arguments: Vec<Argument> = Vec::new();

    let arg_bytes = args.as_bytes();

    for (i, letter) in arg_bytes.iter().enumerate() {
        if letter == &b'-' && arg_bytes[i + 1] == b'-' {
            let remaining_args_str = &args[i + 2..];
            let split_arg_str: Vec<&str> = remaining_args_str.split("-").collect();
            let this_arg_str = split_arg_str[0];

            let trimmed = this_arg_str.trim();

            let split_command = trimmed.split_once(' ');
            let mut arg_name = trimmed;
            let mut args = "";
            if split_command != None {
                (arg_name, args) = split_command.unwrap();
            }

            arguments.push(Argument {
                arg_name: arg_name.to_owned(),
                parameters: args.split(' ').into_iter().map(|a| a.to_owned()).collect(),
            });
        }
    }

    return arguments;
}
