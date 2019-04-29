fn main() {
    /*
     * Declaring variables with different datatypes
     */
    // Implicit declaration
    let val = 19;
    println!("i64: {}", val);

    // Explicit integer declaration with type annotation
    let val: i32 = 20;
    println!("i32: {}", val);

    // Other types

    let val = 3.14; // f64
    println!("f64: {}", val);
    let val: f32 = 3.14; // f32
    println!("f32: {}", val);

    let val = true; // boolean
    println!("boolean: {}", val);
    let val: bool = false;
    println!("boolean: {}", val);

    let val: u32 = 120; // unsigned
    println!("unsigned i32: {}", val);
    let val: i64 = -120; // signed
    println!("signed i64: {}", val);

    let val: u8 = 255; // Smaller integer types
    println!("u8 (Small integers): {}", val);

    // char type in rust represent a unicode scalar type
    let val: char = 'A'; // char type
    println!("char: {}", val);

    // Platform dependent integers
    let val: isize = 10000000; // 64-bit for x86_64 and 32-bit for x86
    println!("isize: {}", val);

    /*
     * Integer literals in Rust
     */
    let decimal: usize = 100_000;
    println!("A decimal: {}", decimal);

    let hex: usize = 0xffd700;
    println!("A hexadecimal: 0x{0:x} i.e {0}", hex);

    let octal: usize = 0o100;
    println!("An octal: 0o{0:o} i.e {0}", octal);

    let bin: usize = 0b0001_0010_0111;
    println!("A binary: {0:b} i.e {0}", bin);

    // Bytes can be of type `u8` only!
    // Hence, using the implicit declaration
    let byte = b'A';
    println!("A byte: {}", byte);

    // Compound data-types

    // Tuple
    let tup = (42, 12, 11);
    let (e1, e2, e3) = tup;
    println!("The value of tup is ({}, {}, {})", e1, e2, e3);

    // Tuple with different types
    let tup: (i32, f64, char) = (500, 6.21, 'A');

    let e1 = tup.0;
    let e2 = tup.1;
    let e3 = tup.2;

    println!("The value of combined tup is ({}, {}, {})", e1, e2, e3);

    // Arrays
    // All array elements are of single type
    let arr = [1, 2, 3, 4];
    println!("The value of arr is {:?}", arr);

    let arr = ["Jan", "Feb", "Mar"];
    println!("The value of arr is {:?}", arr);

    // Defining type and size of an array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is {:?}", arr);

    // Initalizing an array of a certain size with a default value
    let arr = [10; 5]; // [10, 10, 10, 10, 10];
    println!("The value of arr is {:?}", arr);

    // Accesing array elements
    // Out of bounds index causes runtime error
    println!("First Element is {}\nSecond Element is {}", arr[0], arr[1]);
}
