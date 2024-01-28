fn main() {
    // Demonstrating basic arithmetic operations in Rust.

    // Addition of unsigned 32-bit integers
    // This will print "1 + 2 = 3"
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtraction of signed 32-bit integers
    // This will print "1 - 2 = -1"
    println!("1 - 2 = {}", 1i32 - 2);

    // Subtraction of unsigned integers (note: subtraction can underflow)
    // This will print "1 - 2 = 4294967295" due to underflow as unsigned integers can't be negative
    println!("1 - 2 = {}", 1u32.wrapping_sub(2));

    // Demonstrating scientific notation
    // This will print "1e4 is 10000, -2.5e-3 is -0.0025"
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    // This will print "true AND false is false"
    println!("true AND false is {}", true && false);
    // This will print "true OR false is true"
    println!("true OR false is {}", true || false);
    // This will print "NOT true is false"
    println!("NOT true is {}", !true);

    // Demonstrating bitwise operations
    // Binary AND operation (bitwise), prints "0011 AND 0101 is 0001"
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // Binary OR operation (bitwise), prints "0011 OR 0101 is 0111"
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // Binary XOR operation (bitwise), prints "0011 XOR 0101 is 0110"
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // Binary Left Shift operation, prints "1 << 5 is 32"
    println!("1 << 5 is {}", 1u32 << 5);
    // Binary Right Shift operation, prints "0x80 >> 2 is 0x20"
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Demonstrating the use of underscores for readability in large numbers
    // This will print "One million is written as 1000000"
    println!("One million is written as {}", 1_000_000u32);
}
