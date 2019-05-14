fn main() {
    for i in 0..10 {
        print!("{} ", fib(i));
    }
    println!("");

    for i in 0..10 {
        print!("{} ", ifib(i));
    }
    println!("");
}

fn fib(x: usize) -> usize {
    if x == 0 || x == 1 {
        return x;
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn ifib(x: usize) -> usize {
    let mut vals = (0, 1);
    for _ in 0..x {
        vals = (vals.1, vals.0 + vals.1);
    }
    vals.0
}
