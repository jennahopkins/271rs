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
    if *e == BigInt::zero() {
        return BigInt::one();
    }
    let mut t = expmod(b_val, &(e / 2), m);
    t = (t.clone() * t.clone()) % m;
    if e & BigInt::one() == BigInt::one() {
        t = (t * b_val) % m;
    }
    return t;
}

// inv(x:int, q: &BigInt) -> int
pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {
    return expmod(x, &(q - 2), q)
}

// xrecover helper (nested for local use in setup and decode)
pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, i_const: &BigInt) -> BigInt {
    let xx = (y * y - 1) * inv(&(d * y * y + 1), q);
    let mut x = expmod(&xx, &((q + 3) / 8), q);
    if ((x.clone() * x.clone() - xx.clone()) % q) != BigInt::zero() {
        x = (x * i_const) % q;
    }
    if (x.clone() % 2) != BigInt::zero() {
        x = q - x;
    }
    return x;
}

// --- Core Functions (Require Constants) ---

fn edwards(p: &Vec<BigInt>, q_val: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let x1 = &p[0];
    let y1 = &p[1];
    let x2 = &q_val[0];
    let y2 = &q_val[1];
    let x3 = (x1.clone() * y2.clone() + x2.clone() * y1.clone()) * inv(&(BigInt::one() + d * x1.clone() * x2.clone() * y1.clone() * y2.clone()), q);
    let y3 = (y1.clone() * y2.clone() + x1.clone() * x2.clone()) * inv(&(BigInt::one() - d * x1.clone() * x2.clone() * y1.clone() * y2.clone()), q);
    return [x3 % q, y3 % q].to_vec();
}

fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    if *e == BigInt::zero() {
        return [BigInt::zero(), BigInt::one()].to_vec();
    }
    let mut big_q = scalarmult(p, &(e / 2), q, d);
    big_q = edwards(&big_q, &big_q, q, d);
    if (e & BigInt::one()) == BigInt::one() {
        big_q = edwards(&big_q, p, q, d);
    }
    return big_q;
}

fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::new();
    for i in 0..b {
        bits.push(((y >> i) & BigInt::one()).to_u8().unwrap());
    }
    let mut sum: Vec<u8> = Vec::new();
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
    let x = &p[0];
    let y = &p[1];
    let mut bits: Vec<u8> = Vec::new();
    for i in 0..(b - 1) {
        bits.push(((y.clone() >> i) & BigInt::one()).to_u8().unwrap());
    }
    bits.push((x & BigInt::one()).to_u8().unwrap());
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
    let a = BigInt::from(2_i32.pow((b - 2) as u32));
    let mut sum = BigInt::zero();
    for i in 3..(b - 2) {
        sum += BigInt::from(2_i32.pow(i as u32)) * BigInt::from(bit(&h, i));
    }
    let big_a = scalarmult(b_point, &a, q, d);
    return encodepoint(&big_a, b);
}

fn hint(m: &[u8], b: usize) -> BigInt {
    let h = h(m);  // h is bytes
    let mut sum = BigInt::zero();
    for i in 0..(2 * b) {
        sum += BigInt::from(2_i32.pow(i as u32)) * BigInt::from(bit(&h, i));
    }
    return sum;
}

pub fn signature(m: &[u8], sk: &[u8], pk: &[u8], b: usize, q: &BigInt, l: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let h = h(sk);  // h is bytes
    let mut a = BigInt::zero();
    for i in 3..(b - 2) {
        a += BigInt::from(2_i32.pow(i as u32)) * BigInt::from(bit(&h, i));
    }
    a += BigInt::from(2_i32.pow((b - 2) as u32));
    let mut ex1 = h[b / 2..b].to_vec();
    ex1.extend(m);
    let r = hint(&ex1, b);
    let big_r = scalarmult(b_point, &r, q, d);
    let mut ex2 = encodepoint(&big_r, b).to_vec();
    ex2.extend(pk);
    ex2.extend(m);
    let h_sig = hint(&ex2, b);
    let big_s = (r + h_sig * a) % l;
    let mut result = encodepoint(&big_r, b);
    result.extend(encodeint(&big_s, b));
    return result;
}

fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {
    let x = &p[0];
    let y = &p[1];
    return (-x.clone() * x.clone() + y.clone() * y.clone() - 1 - d * x.clone() * x.clone() * y.clone() * y.clone()) % q == BigInt::zero();
}

fn decodeint(s: &[u8], b: usize) -> BigInt {
    let mut sum = BigInt::zero();
    for i in 0..b {
        sum += BigInt::from(2_i32.pow(i as u32)) * BigInt::from(bit(s, i));
    }
    return sum;
}

fn decodepoint(s: &[u8], b: usize, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let mut y = BigInt::zero();
    for i in 0..(b - 1) {
        y += BigInt::from(2_i32.pow(i as u32)) * BigInt::from(bit(s, i));
    }
    let mut x = xrecover(&y, q, d, &BigInt::one());
    if (x.clone() & BigInt::one()) != bit(s, b - 1).into() {
        x = q - x;
    }
    let p = vec![x, y];
    if !isoncurve(&p, q, d) {
        println!("decoding point that is not on curve");
    }
    return p;
}

pub fn checkvalid(s: &[u8], m: &[u8], pk: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt, b_point: &Vec<BigInt>) -> bool {
    if s.len() != b / 4 {
        println!("signature length is wrong");
    }
    if pk.len() != b / 8 {
        println!("public-key length is wrong");
    }

    let big_r = decodepoint(&s[0..b / 8], b, q, d);
    let big_a = decodepoint(&pk, b, q, d);
    let big_s = decodeint(&s[b / 8..b / 4], b);
    let mut ex = encodepoint(&big_r, b);
    ex.extend(pk);
    ex.extend(m);
    let h = hint(&ex, b);
    return scalarmult(b_point, &big_s, q, d) == edwards(&big_r, &scalarmult(&big_a, &h, q, d), q, d);
}