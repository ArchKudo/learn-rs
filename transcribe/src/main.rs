fn transcribe(dna: &str) -> String {
    let allowed = vec!['A', 'C', 'G', 'T'];
    
    dna.to_uppercase()
        .chars()
        .filter(|c| allowed.contains(c))
        .collect::<String>()
        .replace("T", "U")
}
fn main() {
    let test = "atgct123ATCGTTTTTC";
    println!("{}", transcribe(test))
}
