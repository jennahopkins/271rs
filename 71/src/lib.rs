#![allow(non_camel_case_types)]
pub struct ix {
    pub sign: bool, // true = positive, false = negative
    pub vals: Vec<u64>,
}

pub fn add_ix(a: &ix, b: &ix) -> ix {
    /* 
    Addition of two ix numbers.
    Args:
        a: first ix number
        b: second ix number

    Returns:
        ix: result of addition
    */
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
    /*
    Subtraction of two ix numbers.
    Args:
        a: first ix number (minuend)
        b: second ix number (subtrahend)
    
    Returns:
        ix: result of subtraction
     */
    let b = ix {
        sign: !b.sign,
        vals: b.vals.clone(),
    };
    return add_ix(&a, &b);
}

pub fn mul_ix(a: &ix, b: &ix) -> ix {
    // multiply two ix numbers
    let vec: Vec<u64> = mul_mag(&a.vals, &b.vals);
    return ix {
        sign: if a.sign == b.sign { true } else { false },
        vals: vec,
    };
}

fn add_mag(aug_vals: &Vec<u64>, add_vals: &Vec<u64>) -> Vec<u64> {
    /*
    Addition of two magnitudes.

    Args:
        aug_vals: u64 vector chunk values of the augend
        add_vals: u64 vector chunk values of the addend

    Returns:
        Vec<u64>: resulting u64 vector chunk values after addition
    */
    let mut carry = 0u64;
    let mut result: Vec<u64> = Vec::new();
    let max_len: usize = std::cmp::max(aug_vals.len(), add_vals.len());

    // iterate from least significant to most significant chunk
    for i in 0..max_len {
        let aval: u64 = if i < aug_vals.len() { aug_vals[aug_vals.len() - 1 - i] } else { 0 };
        let bval: u64 = if i < add_vals.len() { add_vals[add_vals.len() - 1 - i] } else { 0 };
        
        let sum: u128 = (aval as u128) + (bval as u128) + (carry as u128);
        let sum_minus_carry= sum & 0xFFFFFFFFFFFFFFFF;
        result.push(sum_minus_carry as u64); // push in order of least to most significant
        carry = (sum >> 64) as u64;
    }
    if carry > 0 {
        result.push(carry);
    }

    result.reverse(); // reverse to most significant to least significant
    result
}

fn sub_mag(min_vals: &Vec<u64>, sub_vals: &Vec<u64>) -> Vec<u64> {
    /*
    Subtraction of two magnitudes.

    Args:
        min_vals: u64 vector chunk values of the minuend
        sub_vals: u64 vector chunk values of the subtrahend

    Returns:
        Vec<u64>: resulting u64 vector chunk values after subtraction
    */
    let mut borrow = 0u64;
    let mut result: Vec<u64> = Vec::new();
    let max_len: usize = std::cmp::max(min_vals.len(), sub_vals.len());

    // iterate from least significant to most significant chunk
    for i in 0..max_len {
        let mval: u64 = if i < min_vals.len() { min_vals[min_vals.len() - 1 - i] } else { 0 };
        let sval: u64 = if i < sub_vals.len() { sub_vals[sub_vals.len() - 1 - i] } else { 0 };

        // subtract borrow from minuend first, then subtract subtrahend (overflow means underflow in this case)
        let (intermediate, overflow1) = mval.overflowing_sub(borrow);
        let (final_diff, overflow2) = intermediate.overflowing_sub(sval);

        // if underflow occurred in either subtraction, borrow from next chunk
        borrow = if overflow1 || overflow2 { 1 } else { 0 };

        result.push(final_diff); // push in order of least to most significant
    }

    result.reverse(); // reverse to most significant to least significant
    result
} 

fn gte_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> bool {
    /*
    Check if a is greater than or equal to b.
    Args:
        a_vals: u64 vector chunk values of the first number
        b_vals: u64 vector chunk values of the second number

    Returns:
        bool: true if a >= b, false otherwise
     */
    if a_vals.len() != b_vals.len() {
        // if lengths are different, the longer one is greater
        return a_vals.len() > b_vals.len();
    }

    // compare chunks from most significant to least significant
    for i in 0..a_vals.len() {
        if a_vals[i] != b_vals[i] {
            return a_vals[i] > b_vals[i];
        }
    }
    return true; // they are equal
}

fn mul_mag(a_vals: &Vec<u64>, b_vals: &Vec<u64>) -> Vec<u64> {
    /*
    Multiplication of two magnitudes using long multiplication.

    Args:
        a_vals: u64 vector chunk values of the first number
        b_vals: u64 vector chunk values of the second number

    Returns:
        Vec<u64>: resulting u64 vector chunk values after multiplication
     */
    let mut result: Vec<u64> = vec![0u64; a_vals.len() + b_vals.len()]; // max possible length

    // iterate over each chunk from least significant to most significant
    for (i, &aval) in a_vals.iter().rev().enumerate() {
        let mut carry = 0u128;
        for (j, &bval) in b_vals.iter().rev().enumerate() {
            let idx = result.len() - 1 - (i + j); // index to keep track of significance
            let prod: u128 = (aval as u128) * (bval as u128) + (result[idx] as u128) + carry;
            result[idx] = (prod & 0xFFFFFFFFFFFFFFFF) as u64; // store lower 64 bits
            carry = prod >> 64; // carry upper bits
        }
        // if there's any remaining carry, add it to the next chunk
        if carry > 0 {
            let idx = result.len() - 1 - (i + b_vals.len());
            result[idx] = result[idx].wrapping_add(carry as u64);
        }
    }

    result
}