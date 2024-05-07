fn tm(dna: &str) -> usize {
    let gc = dna
        .to_uppercase()
        .chars()
        .filter(|c| vec!['C', 'G'].contains(c))
        .collect::<String>()
        .len();
    4 * gc + 2 * (dna.len() - gc)
}

fn main() {
    let dna = "ATTT";
    println!("{}", tm(dna))
}
