
fn main() {
    /*let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let data: Vec<u8> = fs::read(filename).unwrap();*/
    let output = base85("man".as_bytes());
    let s = format!("<~{}~>", output);
    for i in 0..(s.len() / 80) {
        println!("{}", &s[i * 80..(i + 1) * 80]);
    }
    print!("{}", &s[s.len() - (s.len() % 80)..]);

}


fn base85() {
    let input = "Hi my name is Jenna".to_string().into_bytes();

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
    println!("{}", output);
}
