use std::io;

fn main() {
    println!("Enter temperature:");
    let mut val = String::new();
    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read number!");
    let val: i32 = val.trim().parse().expect("Enter a valid number!");

    println!("Enter scale:");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read scale!");
    // `to_lowercase` - Converts string to lowercase
    // `chars` - Returns an iterator
    // `nth` - Get value of nth index of iterator
    // `unwrap` - Required(?), More in enums
    let temp = temp.to_lowercase().chars().nth(0).unwrap();

    if temp == 'f' {
        println!("{:?}°F = {:?}°C", val, f2c(val));
    } else if temp == 'c' {
        println!("{:?}°C = {:?}°F", val, c2f(val));
    }
}

// fn func(param: type) -> return_type
fn f2c(temp_f: i32) -> i32 {
    // Implicitly return expressions
    (temp_f - 32) * 5 / 9
}

fn c2f(temp_c: i32) -> i32 {
    (temp_c - 32) * 9 / 5
}
