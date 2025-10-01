fn main() {
    let n: i32 = 3;
    let x = i32_to_f16(n);
    println!("{:?}", x);
}

fn i32_to_f16(n: i32) -> f16::f16 {
    let mut x = f16::f16 {
        sign: 0,
        exp: 0,
        mantissa: 0,
    };

    x.sign = if n < 0 {1} else {0};
    let int = n.clone();

    let mut msb_index: i32 = 0;
    for i in (0..32).rev() {
        if int >> i == 1 {
            msb_index = i;
            break;
        }
    }

    x.exp = (15 + msb_index) as u16;

    let shift: i32 = msb_index - 10;
    x.mantissa = if shift > 0 {
        ((int >> shift) & 0x3FF) as u16
    } else {
        ((int << (-shift)) & 0x3FF) as u16
    };

    return x;
}

// fn println_f16(x: f16) {

// }
