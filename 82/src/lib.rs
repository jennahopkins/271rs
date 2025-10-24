use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive, Euclid};
use num_integer::Integer;
use sha2::{Digest, Sha512};

// --- Global Helpers (No Dependencies) ---

// H(m: bytes) -> bytes
fn h(m: &[u8]) -> Vec<u8> {
    return Sha512::digest(m).to_vec();
    //return Sha512::new().chain_update(m).finalize().to_vec();
}

// bit(h: bytes, i: int) -> int
fn bit(h_val: &[u8], i: usize) -> u8 {
    return (h_val[i / 8] >> (i % 8)) & 1;
}

// expmod(b:int,e:int,m:int) -> int
pub fn expmod(b_val: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    if (e == 0) {
        return BigInt::one();
    }
    let mut t = expmod(b_val, e / 2, m);
    t = (t * t) % m;
    if e & 1 == 1 {
        t = (t * b_val) % m;
    }
    return t;
}

// inv(x:int, q: &BigInt) -> int
pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {
    return expmod(x, q - 2, q)
}

// xrecover helper (nested for local use in setup and decode)
pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, i_const: &BigInt) -> BigInt {
    let xx = (y * y - 1) * inv(d * y * y + 1);
    let mut x = expmod(xx, (q + 3) / 8, q);
    if ((x * x - xx) % q) != 0 {
        x = (x * i_const) % q;
    }
    if (x % 2) != 0 {
        x = q - x;
    }
    return x;
}

// --- Core Functions (Require Constants) ---

fn edwards(p: &Vec<BigInt>, q_val: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let x1 = p[0];
    let y1 = p[1];
    let x2 = q_val[0];
    let y2 = q_val[1];
    let x3 = (x1 * y2 + x2 * y1) * inv(1 + d * x1 * x2 * y1 * y2);
    let y3 = (y1 * y2 + x1 * x2) * inv(1 - d * x1 * x2 * y1 * y2);
    return [x3 % q, y3 % q];
}

fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    if (e == 0) {
        return [BigInt::zero(), BigInt::one()];
    }
    let mut big_q = scalarmult(p, e / 2, q, d);
    big_q = edwards(&big_q, &big_q, q, d);
    if (e & 1) == 1 {
        big_q = edwards(&big_q, p, q, d);
    }
    return big_q;
}

fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::new();
    for i in 0..b {
        bits.push((y >> i) & 1);
    }
    let sum: Vec<u8> = Vec::new();
    for k in 0..(b / 8) {
        let mut byte: u8 = 0;
        for j in 0..8 {
            byte += bits[k * 8 + j] << j;
        }
        sum.push(byte);
    }
    return sum;
}

fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {
    let x = p[0];
    let y = p[1];
    let mut bits: Vec<u8> = Vec::new();
    for i in 0..(b - 1) {
        bits.push((y >> i) & 1);
    }
    bits.push(x & 1);
    let mut sum: Vec<u8> = Vec::new();
    for i in 0..(b / 8) {
        let mut byte: u8 = 0;
        for j in 0..8 {
            byte += bits[i * 8 + j] << j;
        }
        sum.push(byte);
    }
    return sum;
}

pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let h = h(sk);  // h is bytes
    let mut a = 2 ** (b - 2);
    let mut sum = 0;
    for i in 3..(b - 2) {
        sum += 2**i * bit(h, i);
    }
    let big_a = scalarmult(b_point, a, q, d);
    return encodepoint(&big_a, b);
}

fn hint(m: &[u8], b: usize) -> BigInt {
    let h = h(m);  // h is bytes
    let mut sum = 0;
    for i in 0..(2 * b) {
        sum += 2**i * bit(h, i);
    }
    return sum;
}

pub fn signature(m: &[u8], sk: &[u8], pk: &[u8], b: usize, q: &BigInt, l: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let h = h(sk);  // h is bytes
    let mut a = 0;
    for i in 3..(b - 2) {
        a += 2**i * bit(h, i);
    }
    a += 2 ** (b - 2);
    let r = hint(&h[b / 8..b / 4], b) + m;
    let big_r = scalarmult(b_point, &r, q, d);
    let h_sig = hint(&encodepoint(&big_r, b) + pk + m, b);
    let big_s = (r + h_sig * a) % l;
    return encodepoint(&big_r, b) + encodeint(&big_s, b);
}

fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {
    let x = p[0];
    let y = p[1];
    return (-x * x + y * y - 1 - d * x * x * y * y) % q == 0;
}

fn decodeint(s: &[u8], b: usize) -> BigInt {
    let mut sum = 0;
    for i in 0..b {
        sum += 2**i * bit(s, i);
    }
    return sum;
}

fn decodepoint(s: &[u8], b: usize, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let mut y = 0;
    for i in 0..(b - 1) {
        y += 2**i * bit(s, i);
    }
    let mut x = xrecover(y);
    if (x & 1) != bit(s, b - 1) {
        x = q - x;
    }
    let p = vec![x, y];
    if !isoncurve(&p, q, d) {
        panic!("decoding point that is not on curve");
    }
    return p;
    
    /*let mut bits: Vec<u8> = Vec::new();
    for i in 0..(b - 1) {
        bits.push(bit(s, i));
    }
    let mut y = 0;
    for i in 0..(b - 1) {
        y += bits[i] << i;
    }
    let x = xrecover(&y, q, d, i_const);
    if (x & 1) != bit(s, b - 1) {
        x = q - x;
    }
    let p = vec![x % q, y % q];
    if !isoncurve(&p, q, d) {
        panic!("decoding point that is not on curve");
    }
    return p;*/
}

pub fn checkvalid(s: &[u8], m: &[u8], pk: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt, b_point: &Vec<BigInt>) -> bool {
    if (len(s) != b / 4) {
        raise Exception("signature length is wrong");
    }
    if (len(pk) != b / 8) {
        raise Exception("public-key length is wrong");
    }

    let big_r = decodepoint(s[0 : b / 8], b, q, d);
    let big_a = decodepoint(pk, b, q, d);
    let big_s = decodeint(s[b / 8 : b / 4], b, q, d);
    let h = hint(encodepoint(&big_r, b) + pk + m, b);
    return scalarmult(b_point, &big_s, q, d) == edwards(big_r, scalarmult(big_a, h, q, d), q, d);
}