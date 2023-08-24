use crate::rijndael::{decrypt_aes_128, encrypt_aes_128};
use std::error::Error;
use std::fs;
use std::io::Write;

#[derive(Copy, Clone)]
pub enum Command {
    Encrypt,
    Decrypt,
}

pub struct Config {
    pub command: Command,
    pub input_file: String,
    pub output_file: String,
    pub key: u128,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 5 {
            return Err("Incorrect number of arguments given.");
        }

        let command_string = args[1].clone();
        let command: Command;

        if command_string == "encrypt" {
            command = Command::Encrypt
        } else if command_string == "decrypt" {
            command = Command::Decrypt
        } else {
            return Err("Command must be encrypt or decrypt.");
        }
        let input_file = args[2].clone();
        let output_file = args[3].clone();

        let key_bytes = args[4].as_bytes();
        if key_bytes.len() != 16 {
            return Err("Key must be 128 bits.");
        }
        // iterate and join to form u128
        let mut key: u128 = 0;
        for i in 0..16 {
            key = key << 8;
            key |= key_bytes[i] as u128;
        }

        Ok(Config {
            command,
            input_file,
            output_file,
            key,
        })
    }
}

struct FileData {
    data: Vec<u128>,
    padded_bytes: usize,
}

fn load_file(file_path: String, command: &Command) -> Result<FileData, Box<dyn Error>> {
    let mut file_data = fs::read(file_path)?;
    println!("Actual size of file in bytes: {}", file_data.len());
    // for byte in file_data.clone() {
    //     print!("{:#04x} ", byte);
    // }
    // println!("");
    
    let padded_bytes: Option<u8> = match command {
        Command::Decrypt => file_data.pop(),
        Command::Encrypt => None,
    };
    
    let mut file_data_u128: Vec<u128> = Vec::new();
    
    let file_max_len = file_data.len();
    let iter_steps = file_max_len / 16;
    let remaining_bytes = file_max_len % 16;

    for i in 0..iter_steps {
        let mut block_u128 = 0u128;
        for j in 0..16 {
            block_u128 |= (file_data[4 * i + j] as u128) << (8 * (15 - j));
        }
        file_data_u128.push(block_u128);
    }

    match command {
        Command::Decrypt => {
            if remaining_bytes != 0 {
                return Err(
                    "File size is not like `16x + 1` which is required for decryption.".into(),
                );
            }
        }
        Command::Encrypt => {
            if remaining_bytes != 0 {
                // push remaining bytes along with zeros
                let mut remaining_bytes_u128: u128 = 0;
                for i in 0..remaining_bytes {
                    remaining_bytes_u128 = remaining_bytes_u128 << 8;
                    remaining_bytes_u128 |= file_data[16 * iter_steps + i] as u128;
                }
                file_data_u128.push(remaining_bytes_u128 << (128 - 8 * remaining_bytes));
            }
        }
    }

    return Ok(FileData {
        data: file_data_u128,
        padded_bytes: padded_bytes.unwrap_or(remaining_bytes as u8) as usize,
    });
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_file_content = load_file(config.input_file, &config.command)?;
    println!(
        "Loaded file with {} u128s and {} bytes of padding.",
        input_file_content.data.len(),
        input_file_content.padded_bytes
    );
    let key = config.key;

    let mut output_file_data_u8: Vec<u8> = Vec::new();

    match config.command {
        Command::Encrypt => {
            for input_block in input_file_content.data {
                // println!("Input: {:#034x}", input_block);
                let encrypted_block = encrypt_aes_128(input_block, key);
                // println!("Encrypted: {:#034x}", encrypted_block);
                // break u128 into 16 u8 and then push
                for i in (0..128).step_by(8).rev() {
                    // println!("{:010x}", (encrypted_block >> i & 0xFF) as u8);
                    output_file_data_u8.push((encrypted_block >> i & 0xFF) as u8);
                }
            }
            // add byte to end telling how much zero padding was done
            output_file_data_u8.push(input_file_content.padded_bytes as u8);
        }
        Command::Decrypt => {
            let padded_bytes = input_file_content.padded_bytes;
            for input_block in input_file_content.data {
                // println!("Input: {:#034x}", input_block);
                let decrypted_block = decrypt_aes_128(input_block, key);
                // println!("Decrypted: {:#034x}", decrypted_block);
                // break u128 into 16 u8 and then push
                for i in (0..128).step_by(8).rev() {
                    // println!("{:010x}", (decrypted_block >> i & 0xFF ) as u8);
                    output_file_data_u8.push((decrypted_block >> i & 0xFF) as u8);
                }
            }
            println!("Padded bytes: {}", padded_bytes);
            for _i in 0..padded_bytes {
                // pop all padded bytes at end
                let _ = output_file_data_u8.pop();
            }
        }
    }
    println!("Outfile size in bytes: {}", output_file_data_u8.len());
    // for byte in output_file_data_u8.clone() {
    //     print!("{:#04x} ", byte);
    // }
    // println!("");
    let mut outfile_buffer = fs::File::create(config.output_file)?;
    outfile_buffer.write_all(&output_file_data_u8)?;

    Ok(())
}
