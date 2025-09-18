// can i make them mutable?
pub fn weight_u8(byte: u8) -> u64 {
    /* Finds the hamming weight of a u8 */
    let mut byte: u8 = byte.clone();
    let mut count: u64 = 0;
    while byte != 0 {
        count += (byte & 1) as u64;
        byte >>= 1;
    }
    return count;
}

pub fn weight_u64(word: u64) -> u64 {
    /* Finds the hamming weight of a u64 */
    let mut word: u64 = word.clone();
    let mut count: u64 = 0;
    while word != 0 {
        count += (word & 1) as u64;
        word >>= 1;
    }
    return count;
}

pub fn weight_bytes(bytes: Vec<u8>) -> u64 {
    /* Finds the hamming weight of a vector of u8s */
    let mut count: u64 = 0;
    for byte in bytes {
        count += weight_u8(byte);
    }
    return count;
}

pub fn weight_words(words: Vec<u64>) -> u64 {
    /* Finds the hamming weight of a vector of u64s */
    let mut count: u64 = 0;
    for word in words {
        count += weight_u64(word);
    }
    return count;
} 

pub fn distance_u8(a: u8, b: u8) -> u64 {
    /* Finds the hamming distance between two u8s */
    let mut count: u64 = 0;
    count += weight_u8(a ^ b);
    return count;
}

pub fn distance_u64(c: u64, d: u64) -> u64 {
    /* Finds the hamming distance between two u64s */
    let mut count: u64 = 0;
    count += weight_u64(c ^ d);
    return count;
}

pub fn distance_bytes(a_vec: Vec<u8>, b_vec: Vec<u8>) -> u64 {
    /* Finds the hamming distance between two vectors of u8s */
    let mut count: u64 = 0;
    for i in 0..a_vec.len() {
        count += distance_u8(a_vec[i], b_vec[i]);
    }
    return count;
}

pub fn distance_words(c_vec: Vec<u64>, d_vec: Vec<u64>) -> u64 {
    /* Finds the hamming distance between two vectors of u64s */
    let mut count: u64 = 0;
    for i in 0..c_vec.len() {
        count += distance_u64(c_vec[i], d_vec[i]);
    }
    return count;
}

