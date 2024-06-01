fn kmers_loop(dna: &String, kmer: usize) -> Vec<&str> {
    let mut kmers = Vec::new();
    for idx in 0..dna.len() + 1 - kmer {
            kmers.push(&dna[idx..idx + kmer])
    }

    kmers
}

fn main() {
    let dna = String::from("ATCG");
    println!("{:?}", kmers(&dna, 3))
}
