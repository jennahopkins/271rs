#![allow(non_camel_case_types)]
pub struct ix {
    pub sign: bool,
    pub vals: Vec<u64>,
}

pub fn add_ix(a: &ix, b: &ix) -> ix {
    if a.sign == b.sign {
        let vec: Vec<u64> = add_mag(&a.vals, &b.vals);
        return ix {
            sign: a.sign,
            vals: vec,
        };
    } else {
        // Handle subtraction case
        if gte_mag(&a.vals, &b.vals) {
            // a is greater in magnitude
            let vec: Vec<u64> = sub_mag(&a.vals, &b.vals);
            return ix {
                sign: a.sign,
                vals: vec,
            };
        } else {
            // b is greater in magnitude
            let vec: Vec<u64> = sub_mag(&b.vals, &a.vals);
            return ix {
                sign: b.sign,
                vals: vec,
            };
        }
    }
}

pub fn sub_ix(a: &ix, b: &ix) -> ix {
    let b = ix {
        sign: !b.sign,
        vals: b.vals.clone(),
    };
    return add_ix(&a, &b);
}
/* 
pub fn mul_ix(_a: &ix, _b: &ix) -> ix {
    // multiply two ix numbers
    
} */

// Helpers: Add/sub magnitudes (absolute values) of two numbers.

// "aug" and "add" are short for "augend" and "addend"
fn add_mag(aug_vals: &Vec<u64>, add_vals: &Vec<u64>) -> Vec<u64> {
    let mut carry = 0u64;
    let mut result: Vec<u64> = Vec::new();
    let max_len: usize = std::cmp::max(aug_vals.len(), add_vals.len());

    for i in 0..max_len {
        let aval: u64 = if i < aug_vals.len() { aug_vals[aug_vals.len() - 1 - i] } else { 0 };
        let bval: u64 = if i < add_vals.len() { add_vals[add_vals.len() - 1 - i] } else { 0 };
        
        let sum: u128 = (aval as u128) + (bval as u128) + (carry as u128);
        let sum_minus_carry= sum & 0xFFFFFFFFFFFFFFFF;
        result.push(sum_minus_carry as u64);
        carry = (sum >> 64) as u64;
    }
    if carry > 0 {
        result.push(carry);
    }

    result.reverse();
    result
}


// "min" and "sub" are short for "minuend" and "subtrahend"
fn sub_mag(min_vals: &Vec<u64>, sub_vals: &Vec<u64>) -> Vec<u64> {
    let mut borrow = 0u64;
    let mut result: Vec<u64> = Vec::new();
    let max_len: usize = std::cmp::max(min_vals.len(), sub_vals.len());

    for i in 0..max_len {
        let mval: u64 = if i < min_vals.len() { min_vals[min_vals.len() - 1 - i] } else { 0 };
        let sval: u64 = if i < sub_vals.len() { sub_vals[sub_vals.len() - 1 - i] } else { 0 };
        
        /*let diff: u128 = (mval as u128) - (sval as u128) - (borrow as u128);
        let diff_minus_borrow = diff & 0xFFFFFFFFFFFFFFFF;
        result.push(diff_minus_borrow as u64);
        borrow = if diff >> 64 == 1 { 1 } else { 0 };*/

        let (intermediate, overflow1) = mval.overflowing_sub(borrow);
        let (final_diff, overflow2) = intermediate.overflowing_sub(sval);

        borrow = if overflow1 || overflow2 { 1 } else { 0 };
        result.push(final_diff);
    }

    result.reverse();
    return result;
} 



// Compute the "greater than or equal" between two values.
fn gte_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> bool {
    if a_vals.len() != b_vals.len() {
        return a_vals.len() > b_vals.len();
    }
    for i in 0..a_vals.len() {
        if a_vals[i] != b_vals[i] {
            return a_vals[i] > b_vals[i];
        }
    }
    return true; // they are equal
}