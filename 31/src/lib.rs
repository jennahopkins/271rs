pub fn weight_u8(byte: u8) -> u64 {
    let mut count: u64 = 0;
    let bin_str = format!("{:08b}", byte);
    for ch in bin_str.chars() {
        if ch == '1' {
            count += 1;
        }
    }
    return count;
}

pub fn weight_u64(word: u64) -> u64 {
    let mut count: u64 = 0;
    let bin_str = format!("{:64b}", word);
    for ch in bin_str.chars() {
        if ch == '1' {
            count += 1;
        }
    }
    return count;
}

pub fn weight_bytes(bytes: Vec<u8>) -> u64 {
    let mut count: u64 = 0;
    for byte in bytes {
        count += weight_u8(byte);
    }
    return count;
}

pub fn weight_words(words: Vec<u64>) -> u64 {
    let mut count: u64 = 0;
    for word in words {
        count += weight_u64(word);
    }
    return count;
} 
