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
    for j in [2, 3, 5, 7, 11, 13, 15, 17, 19] {
        let num = constants(j as u64);
        println!("sqrt({j}) = {:016x}", num);
    }
    /*
    println!("sqrt(02) = {}", constants(2 as u64)); // 01fffffffa7a8770a4
    println!("sqrt(03) = {}", constants(3 as u64)); // 02fffffff746605709
    println!("sqrt(05) = {}", constants(5 as u64)); // 04fffffff29bbdd100
    println!("sqrt(07) = {}", constants(7 as u64)); // 06ffffffee28d451d1
    println!("sqrt(11) = {}", constants(11 as u64)); // 0affffffe79823ac10
    println!("sqrt(13) = {}", constants(13 as u64)); // 0cffffffe1effec840
    println!("sqrt(17) = {}", constants(17 as u64)); // 10ffffffd6ebf68af1
    println!("sqrt(19) = {}", constants(19 as u64)); // 12ffffffd3bf490990
    */
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
    let root: f64 = f64::sqrt(x as f64);
    let int = root as u64;
    let frac_part = root - int as f64;

    let mut approx = (frac_part * (1u128 << 64) as f64) as u64;
    approx = (approx >> 32) << 32;
    let target = ((x as u128 - 1) << 64) | 0xffff_ffff_ffff_ffff;
    for i in (0..32).rev() {
        let one_bit = 1u64 << i;
        if approx & one_bit == 0 {
            let candidate = approx | one_bit;
            let square = (candidate as u128) * (candidate as u128);
            println!(
            "i {:2}: trying {:016x}, square = {:032x}, target = {:032x}",
            i, candidate, square, target
            );

            if square <= target {
                approx = candidate;
            }
        }
    }
    return approx;
}
