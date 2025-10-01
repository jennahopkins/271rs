fn main() {
    for i in [2,3,5,7,11] {
        println!("{i}");
    }
    let mut cnt = 5;
    let mut val = 13;
    while cnt < 8 {
        if is_prime(val) {
            println!("{val}");
            cnt += 1;
        }
        val += 2;
    }
    let num = constants(2 as u64);
    println!("sqrt(2) = {:016x}", num);
    /*for j in [2, 3, 5, 7, 11, 13, 15, 17, 19] {
        let num = constants(j as u64);
        println!("sqrt({j}) = {:016x}", num);
    }*/
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    // Check for divisibility from 2 up to the square root of n.
    // If n is composite, it must have at least one factor less than or equal to its square root.
    let limit = (n as f64).sqrt() as u64;
    for i in 2..=limit {
        if n % i == 0 {
            // If n is divisible by i, it's not prime.
            return false;
        }
    }

    // If no divisors are found, the number is prime.
    true
}

fn constants(x: u64) -> u64{

    let root: f32 = f64::sqrt(x as f64) as f32; // f64::sqrt
    let int = root as u32; // getting the int part
    let frac_part = root - int as f32; // getting the fractional
    println!("frac_part: {frac_part}");
    let fractional: f32 = frac_part * (2_f32.powf(32.0)); // express it in 32 bits
    println!("fractional: {fractional}");
    let mut approx = fractional as u32; // convert to integer type
    println!("approx: {approx:b}");

    // for every of the bits in approx starting with greatest placevalue
    for i in (0..32).rev() {
        let one = 1u32 << i; // the one is in the placevalue targeted
        if approx & one == 0 { // there is a 0 in this placevalue of approx
            let candidate = approx | one; // changing the 0 to a 1
            let square = (candidate as u128) * (candidate as u128); // squaring the candidate as a u128
            if square <= 0x0000000000000000ffffffffffffffff { // if no overflow
                approx = candidate; // keep the one
            }
            println!(
                "i {:2}: candidate {:016x}, square = {:016x}",
                i, candidate, square
                );
        }
    }
    return (approx as u128 * approx as u128) as u64;
}
