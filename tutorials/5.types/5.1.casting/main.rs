// Rust provides no implicit type conversion (coercion) between primitive types.
// But, explicit type conversion (casting) can be performed using the as keyword.

// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules. A float cannot be directly converted to a char.
    // let character = decimal as char;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    // i8 ranges from -128 -> 0 -> 127
    // f(x) -> compliment
    // f(01111111) -> 10000000
    // 8 Bit Binary representation
    // 0111 1111 = 127
    // 0000 0000 = 0
    // 1000 0000 = -128
    // 128 is not possible in a 8 bit binary representation
    // so rust will show it's compliment instead
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);

    // The two's complement of 232 is -24
    // Read example of "128 as u8" above
    // Integers available - 256 - 232 = -24
    // binary representation
    //   24 = 0001 1000
    // - 24 = 1110 1000 not 1110 0111 which is - 25
    //
    // 1111 1111 = -1 with it's complement 0000 0001 = 1
    // 1111 1110 = -2 with it's complement 0000 0010 = 2
    // i.e. it counts backwards from 0 which is 0000 0000 to -1 which is 1111 1111
    println!(" 232 as a i8 is : {}", 232 as i8);
}


