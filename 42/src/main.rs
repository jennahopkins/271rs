use std::env;
use std::fs;


// SHA-512 initial hash values (in big-endian):
const HASHES: [u64; 8] = [
    0x6a09e667f3bcc908, 
    0xbb67ae8584caa73b, 
    0x3c6ef372fe94f82b, 
    0xa54ff53a5f1d36f1, 
    0x510e527fade682d1, 
    0x9b05688c2b3e6c1f, 
    0x1f83d9abfb41bd6b, 
    0x5be0cd19137e2179,
];

// SHA-512 round constants:
const CONSTANTS: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 0x3956c25bf348b538, 
    0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242, 0x12835b0145706fbe, 
    0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 
    0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65, 
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5, 0x983e5152ee66dfab, 
    0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725, 
    0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 
    0x53380d139d95b3df, 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b, 
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218, 
    0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8, 0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 
    0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 
    0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec, 
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b, 0xca273eceea26619c, 
    0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba, 0x0a637dc5a2c898a6, 
    0x113f9804bef90dae, 0x1b710b35131c471b, 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 
    0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
];


fn main() {
    /*
    Main function to read a file and compute its SHA-512 hash.
        Expects a command-line argument for the filename to hash.
        Prints the SHA-512 hash in hexadecimal format followed by the filename.
     */
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let data: Vec<u8> = fs::read(filename).unwrap();

    let digest: [u8; 64] = sha512(&data);
    for byte in &digest {
        print!("{:02x}", byte);
    }
    println!("  {}", filename);
}

fn sha512(text: &[u8]) -> [u8; 64] {
    /*
    Main function to compute the SHA-512 hash of the input text.
    Args:
        text: input message as a byte slice
    Returns:
        64-byte array representing the SHA-512 digest
    */
    let mut w: [u64; 80] = [0u64; 80];
    let mut state: [u64; 8] = HASHES;
    let bitlen: u128 = (text.len() as u128) * 8;
    let mut offset: usize = 0;

    // process all full blocks of the message
    while offset + 128 <= text.len() {
        let mut block: [u8; 128] = [0u8; 128];
        block.copy_from_slice(&text[offset..offset + 128]);
        sha512_compress(&mut w, &mut state, block);
        offset += 128;
    }

    // handle the final partial block(s)
    let mut block: [u8; 128] = [0u8; 128];
    let remaining: &[u8] = &text[offset..];
    let rem_len: usize = remaining.len();

    // copy the remaining bytes and append the '1' bit
    block[..rem_len].copy_from_slice(remaining);
    block[rem_len] = 0x80;

    if rem_len < 112 {
        // there's enough space for length in this block; zero bytes after the 1 to 112 then add length, and process
        for i in (rem_len + 1)..112 {
            block[i] = 0;
        }

        for i in 0..16 {
            block[112 + i] = ((bitlen >> (8 * (15 - i))) & 0xFF) as u8;
        }
        sha512_compress(&mut w, &mut state, block);
    } else {
        // not enough space for length in this block, zero the rest and process
        for i in (rem_len + 1)..128 {
            block[i] = 0;
        }
        sha512_compress(&mut w, &mut state, block);

        // process a second block to fit length at the end
        let mut block2: [u8; 128] = [0u8; 128];
        for i in 0..16 {
            block2[112 + i] = ((bitlen >> (8 * (15 - i))) & 0xFF) as u8;
        }
        sha512_compress(&mut w, &mut state, block2);
    }

    // produce final digest bytes from state
    let mut digest: [u8; 64] = [0u8; 64];
    for (i, word) in state.iter().enumerate() {
        digest[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
    }

    return digest;

}

fn sha512_compress(w: &mut [u64; 80], state: &mut [u64; 8], block: [u8; 128]) {
    /*
    Helper function to process a single 1024-bit block.

    Args:
        w: mutable reference to the message schedule array (80 u64 words)
        state: mutable reference to the current hash state (8 u64 words)
        block: 128-byte array representing the 1024-bit message block
     */
    let mut temp1: u64;
    let mut temp2: u64;

    // prepare the message schedule
    for i in 0..16 {
        w[i] = ((block[i * 8] as u64) << 56) |
               ((block[i * 8 + 1] as u64) << 48) |
               ((block[i * 8 + 2] as u64) << 40) |
               ((block[i * 8 + 3] as u64) << 32) |
               ((block[i * 8 + 4] as u64) << 24) |
               ((block[i * 8 + 5] as u64) << 16) |
               ((block[i * 8 + 6] as u64) << 8) |
               (block[i * 8 + 7] as u64);
    };

    // extend the first 16 words into the remaining 64 words w[16..79] of the message schedule array
    for i in 16..80 {
        w[i] = sigma1(w[i - 2]) + 
            w[i - 7] + 
            sigma0(w[i - 15]) + 
            w[i - 16];
    }

    // initialize working variables to current hash value
    let mut a: u64 = state[0];
    let mut b: u64 = state[1];
    let mut c: u64 = state[2];
    let mut d: u64 = state[3];
    let mut e: u64 = state[4];
    let mut f: u64 = state[5];
    let mut g: u64 = state[6];
    let mut h: u64 = state[7];

    // compression hash function main loop, 80 rounds
    for i in 0..80 {
        temp1 = h + Sigma1(e) + choice(e, f, g) + CONSTANTS[i] + w[i];
        temp2 = Sigma0(a) + median(a, b, c);
        h = g;
        g = f;
        f = e;
        e = d + temp1;
        d = c;
        c = b;
        b = a;
        a = temp1 + temp2;
    }

    // add the compressed chunk to the current hash value
    state[0] += a;
    state[1] += b;
    state[2] += c;
    state[3] += d;
    state[4] += e;
    state[5] += f;
    state[6] += g;
    state[7] += h;
}


fn Sigma0 (x: u64) -> u64 {
    /* helper function to mix parts of the current hash state to help spread changes */
    return x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39);
}

fn Sigma1 (x: u64) -> u64 {
    /* helper function to add more mixing to the hash state for stronger security */
    return x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41);
}

fn sigma0 (x: u64) -> u64 {
    /* helper function to scramble earlier message words to prepare for hashing */
    return x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7);
}

fn sigma1 (x: u64) -> u64 {
    /* helper function to mixe up message words for better randomness in the schedule */
    return x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6);
}

fn choice (x: u64, y: u64, z: u64) -> u64 {
    /* helper function to compute the choice function */
    return (x & y) ^ ((!x) & z);
}

fn median (x: u64, y: u64, z: u64) -> u64 {
    /* helper function to compute the median function */
    return (x & y) ^ (x & z) ^ (y & z);
}