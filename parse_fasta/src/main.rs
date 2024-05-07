use std::collections::HashMap;
use std::fs;

fn gc_content(dna: &str) -> f32 {
    let gc_count = dna
        .to_uppercase()
        .chars()
        .filter(|c| *c == 'G' || *c == 'C')
        .count() as f32;
    gc_count / dna.len() as f32
}

fn motif(motif: &str, dna: &str) -> i32 {
    dna.to_uppercase()
        .find(motif)
        .map(|idx| idx as i32)
        .unwrap_or(-1)
}

fn summarize(seqs: HashMap<String, String>) -> HashMap<String, (i32, f32)> {
    seqs.into_iter()
        .map(|(key, value)| (key, (motif("TTT", &value), gc_content(&value))))
        .collect()
}

fn ser(filename: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut sequences = HashMap::new();

    for seq in contents.split('>') {
        let mut lines = seq.lines();
        if let Some(key) = lines.next() {
            let value = lines.collect::<Vec<&str>>().join("\n");
            sequences.insert(key.to_string(), value);
        }
    }

    Ok(sequences)
}

fn main() {
    let filename = "../data/ref.fna";
    match ser(filename) {
        Ok(hashmap) => {
            println!("{:?}", summarize(hashmap));
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
