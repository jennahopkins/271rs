use numerical::*;
/*use bignum::*;*/

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a = h2i_ix(&args[1]);
    let b = h2i_ix(&args[2]);
    match args[3].as_str() {
        "ADD" => see_ix(&add_ix(&a, &b)),
        "SUB" => see_ix(&sub_ix(&a, &b)),
        "MUL" => see_ix(&mul_ix(&a, &b)),
        /*"DIV" => todo!(),
        "REM" => todo!(),*/
        &_    => println!("Operator not recognized: choose from ADD, SUB, MUL, DIV, REM"),
    }
}

fn h2i_ix(h: &str) -> ix {
    /*
    Convert a hexadecimal string to an ix (big integer).
    Args:
        h: hexadecimal string
    Returns:
        ix: big integer representation struct
     */
    let hex = h.trim_start_matches("0x").trim_start_matches("0X");
    let mut hex = hex.to_string();

    // Pad so length is a multiple of 16
    let rem = hex.len() % 16;
    if rem != 0 {
        let pad = 16 - rem;
        hex = "0".repeat(pad) + &hex;
    }

    let mut values: Vec<u64> = Vec::new();
    // break into 16-char chunks and convert to u64 starting from least significant
    for chunk in hex.as_bytes().rchunks(16) {
        let chunk_str = std::str::from_utf8(chunk).unwrap();
        let value = u64::from_str_radix(chunk_str, 16).unwrap();
        values.push(value); // push in LSB to MSB order
    }
    values.reverse(); // reverse to MSB to LSB order

    let ix_inst = ix {
        sign: true,
        vals: values,
    };
    ix_inst
}

fn see_ix(x: &ix) {
    /*
    Print the hexadecimal representation of an ix (big integer).
    Args:
        x: big integer to print
     */
    let mut result = String::new();

    for (i, &val) in x.vals.iter().enumerate() {
        if i == 0 {
            // Most significant chunk — no leading zeros
            result += &format!("{:X}", val);
        } else {
            // Other chunks — pad to 16 hex digits
            result += &format!("{:016X}", val);
        }
    }
    println!("{}{}", if x.sign { "" } else { "-" }, result);
}