fn main() {
    // Defining immutable variables with `let`
    let some_val = 100;
    let some_string = "100";

    println!("The value of some_val is {}", some_val);
    println!("The value of some_string is {}", some_string);

    // Defining mutable variables with `mut`
    let mut mutable_var = 120;
    println!("First the value was: {}", mutable_var);
    mutable_var += 10;
    println!("Then it was modified to: {}", mutable_var);

    // Defining constants using `const`
    // Requires type annotations
    const PI: f32 = 3.14;
    println!("The value of Pi is {}", PI);

    // Shadowing immutable variables with `let`
    // Same type
    let val = 100;
    println!("val: {}", val);
    let val = val * 2;
    println!("val: {}", val);
    // Shadowing to different type
    let val = "Hello, World";
    println!("Now a string: {}", val);
    let val = val.len();
    println!("Back to int: {}", val);

    // NOTE: mutable variables / constants can't be shadowed

    // Invalid Example 1
    // const val: type = value;
    // const val: same/other type = value;

    // Invalid Example 2
    // let mut val = value of type X
    // val = value of type Y
}
