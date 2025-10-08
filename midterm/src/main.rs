use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let data: Vec<u8> = fs::read(filename).unwrap();
    let output = base85(&data);
    let s = format!("<~{}~>", output);
    let encoded = base85(&data);
    let wrapped = format!("<~{}~>", encoded);

    for i in 0..(wrapped.len() / 80) {
        println!("{}", &wrapped[i * 80..(i + 1) * 80]);
    }

    if wrapped.len() % 80 != 0 {
        print!("{}", &wrapped[wrapped.len() - (wrapped.len() % 80)..]);
    }

}


fn base85(input: &[u8]) -> String{
    let mut output = String::new();
    let mut i = 0;

    while i < input.len() {
        // prepare 4-byte chunk (pad with 0s if needed)
        let mut buffer: u32 = 0;
        let mut chunk_len = 0;

        for _ in 0..4 {
            buffer <<= 8;
            if i < input.len() {
                buffer |= input[i] as u32;
                chunk_len += 1;
                i += 1;
            }
        }

        // convert buffer to 5 base85 characters
        let mut chars = ['\0'; 5];
        for j in (0..5).rev() {
            chars[j] = ((buffer % 85) as u8 + 33) as char;
            buffer /= 85;
        }

        // don't add padding characters if chunk was less than 4 bytes
        for k in 0..(chunk_len + 1) {
            output.push(chars[k]);
        }
    }
    return output;
}
