fn main() {
    /* Testing statements for hamming functions in lib.rs */
    dbg!(hamming::weight_u8(0x33_u8)); // should be 4
    dbg!(hamming::weight_u8(255)); // should be 8
    dbg!(hamming::weight_u64(0x1A2B3C4D5E6F7081)); // should be 31
    let bytes: Vec<u8> = vec![72, 101, 108, 108, 111];
    dbg!(hamming::weight_bytes(bytes.clone())); // should be 20
    let words: Vec<u64> = vec![42, 123456789, 999999999999, 18446744073709551615];
    dbg!(hamming::weight_words(words.clone())); // should be 107
    dbg!(hamming::distance_u8(0x33, 255)); // should be 4
    dbg!(hamming::distance_u64(0x1A2B3C4D5E6F7081, 18446744073709551615)); // should be 33
    dbg!(hamming::distance_bytes(bytes.clone(), bytes)); // should be 0
    let words2: Vec<u64> = vec![41, 123456789, 999999999999, 18446744073709551615];
    dbg!(hamming::distance_words(words, words2)); // should be 2
}
