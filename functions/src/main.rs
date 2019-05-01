fn main() {
    hello_function();
    print_twice(10);
    println!("The answer is {}", forty_two());
    println!("Is the answer 21? {}", is_answer(21));
    println!("Is the answer 42? {}", is_answer(42));
}

// Functions begin with the fn keyword
fn hello_function() {
    println!("Hello, world!");
}

// Function with parameters
fn print_twice(x: isize) {
    println!("{0} {0}", x);
}

// Implicit return
fn forty_two() -> isize {
    5
}

// Explicit return
fn is_answer(x: isize) -> bool {
    if x == 42 {
        return true;
    }
    false
}
