fn main() {
    let num = 5;
    // Using if
    if num < 10 {
        println!("Colder");
    } else {
        println!("Warmer");
    }

    // Using if with multiple else-if arms
    if num < 10 {
        println!("Colder");
    } else if num == 10 {
        println!("Perfection.");
    } else {
        println!("Warmer");
    }

    // Conditional assignment
    let half = if num % 2 == 0 { num / 2 } else { (num + 1) / 2 };

    println!("Half: {}", half);

    // Using loops
    let mut counter = 0;
    let result = loop {
        println!("Calculating...");
        counter += 1;
        if counter == 10 {
            // also returns
            break counter;
        }
    };
    println!("The result is {}", result);

    // Using while
    let mut countdown = 10;
    while countdown != 0 {
        print!("{}.. ", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // Iterating an array using while
    let arr = [10; 5];
    let mut idx = 0;
    while idx < arr.len() {
        print!("{} ", arr[idx]);
        idx += 1;
    }
    println!("are the elements of the array.");

    // Iterating an array using for
    for i in arr.iter() {
        print!("{} ", i);
    }
    println!("are the elements of the array.");

    // Reverse iterating using rev()
    for i in (1..10 + 1).rev() {
        print!("{}.. ", i);
    }
    println!("Liftoff!");
}
