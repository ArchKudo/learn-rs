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

// Not entirely sure how yield works and, not rolling a iterator
// just yet... So ya, beware of a subtle bug in this function!
fn ifib(x: usize) -> usize {
    let mut vals = (0, 1);
    for _ in 1..x {
        vals = (vals.1, vals.0 + vals.1);
    }
    vals.1
}
