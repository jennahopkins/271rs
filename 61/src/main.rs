use std::env;
use std::fs;

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    /*
    Read a file and encode its contents to Base64.
     */
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let data: Vec<u8> = fs::read(filename).unwrap();

    for (i, ch) in base64(&data).chars().enumerate() {
        if i % 76 == 0 && i != 0 {
            println!();
        }
        print!("{}", ch);
    }
    println!();
}

fn base64(input: &[u8]) -> String{
    /*
    Main function to encode input bytes to Base64.
    Args:
        input: byte slice to encode
    Returns:
        String: Base64 encoded string
     */
    let mut output = String::new();
    let mut buffer = 0u64;
    let mut bits_collected = 0;

    for &byte in input {
        buffer = (buffer << 8) | byte as u64;
        bits_collected += 8;

        // taking 6-bit chunks and encoding them to Base64 characters
        while bits_collected >= 6 {
            bits_collected -= 6;
            let index = ((buffer >> bits_collected) & 0b111111) as usize;
            output.push(BASE64_CHARS[index] as char);
        }
    }

    // handle remaining bits
    if bits_collected > 0 {
        buffer <<= 6 - bits_collected;
        let index = (buffer & 0b111111) as usize;
        output.push(BASE64_CHARS[index] as char);
    }

    // add padding if the last chunk is not a full 24 bits
    while output.len() % 4 != 0 {
        output.push('=');
    }

    return output;
}


