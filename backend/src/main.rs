use std::env;

fn generate_rsa_keys() -> String {
    "public_key private_key".to_string().to_string()
}

fn rsa_encrypt_aes_key(_public_key: &str, aes_key: &str) -> String {
    aes_key.to_string()
}

fn rsa_decrypt_aes_key(_private_key: &str, encrypted_aes_key: &str) -> String {
    encrypted_aes_key.to_string()
}

fn generate_aes_key() -> String {
    "aes_key".to_string()
}

fn aes_encrypt_text(_aes_key: &str, text: &str) -> String {
    text.to_string()
}

fn aes_decrypt_text(_aes_key: &str, encrypted_text: &str) -> String {
    encrypted_text.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("ERROR: command not provided");
        return;
    }

    let command = &args[1];

    let result = match command.as_str() {
        "generate_rsa_keys" => generate_rsa_keys(),

        "rsa_encrypt_aes_key" => {
            if args.len() < 4 {
                "ERROR: expected public_key and aes_key".to_string()
            } else {
                rsa_encrypt_aes_key(&args[2], &args[3])
            }
        }

        "rsa_decrypt_aes_key" => {
            if args.len() < 4 {
                "ERROR: expected private_key and encrypted_aes_key".to_string()
            } else {
                rsa_decrypt_aes_key(&args[2], &args[3])
            }
        }

        "generate_aes_key" => generate_aes_key(),

        "aes_encrypt_text" => {
            if args.len() < 4 {
                "ERROR: expected aes_key and text".to_string()
            } else {
                aes_encrypt_text(&args[2], &args[3])
            }
        }

        "aes_decrypt_text" => {
            if args.len() < 4 {
                "ERROR: expected aes_key and encrypted_text".to_string()
            } else {
                aes_decrypt_text(&args[2], &args[3])
            }
        }

        _ => "ERROR: unknown command".to_string(),
    };

    println!("{}", result);
}