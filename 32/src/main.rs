fn main() {
    /* Various Variables*/
    let a : [u64; 4] = [0x1111111111110000, 0x1111000011001100, 0x1100110010101010, 0x0123456789ABCDEF];

    println!("CHOICE(\n{:016X},\n{:016X},\n{:016X}) = \n--------\n{:016X}\n\n", 
             a[0], a[1], a[2], macros::choice!(a[0], a[1], a[2]));
    println!("MEDIAN(\n{:016X},\n{:016X},\n{:016X}) = \n--------\n{:016X}\n\n", 
             a[0], a[1], a[2], macros::median!(a[0], a[1], a[2]));
    println!("*Rotates use a decimal shift value, but print in hexadecimal:\n");
    println!("ROTATE RIGHT(\n{:016X}, 04) = \n--------\n{:016X}\n\n", a[3],   macros::rotright!(a[3], 4));
    println!("ROTATE RIGHT(\n{:016X}, 08) = \n--------\n{:016X}\n\n", a[3],   macros::rotright!(a[3], 8));
    println!("ROTATE RIGHT(\n{:016X}, 12) = \n--------\n{:016X}\n\n", a[3],   macros::rotright!(a[3], 12));
    println!("ROTATE RIGHT(\n{:016X}, 02) = \n--------\n{:016X}\n\n", 0x1000, macros::rotright!(0x1000, 2));
    println!("ROTATE RIGHT(\n{:016X}, 30) = \n--------\n{:016X}\n\n", 0x1000, macros::rotright!(0x1000, 30));

    println!("ROTATE LEFT(\n{:016X}, 04) = \n--------\n{:016X}\n\n", a[3],   macros::rotleft!(a[3], 4));
    println!("ROTATE LEFT(\n{:016X}, 08) = \n--------\n{:016X}\n\n", a[3],   macros::rotleft!(a[3], 8));
    println!("ROTATE LEFT(\n{:016X}, 12) = \n--------\n{:016X}\n\n", a[3],   macros::rotleft!(a[3], 12));
    println!("ROTATE LEFT(\n{:016X}, 02) = \n--------\n{:016X}\n\n", 0x1000, macros::rotleft!(0x1000, 2));
    println!("ROTATE LEFT(\n{:016X}, 30) = \n--------\n{:016X}\n\n", 0x1000, macros::rotleft!(0x1000, 30));
}
