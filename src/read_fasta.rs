extern crate bio;

use std::collections::HashMap;
use bio::io::fasta;
use std::env;


pub fn read_fasta_core(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>{
    let reader = fasta::Reader::from_file(path).unwrap();
    let mut id2seq: HashMap<String, String> = HashMap::new();
    for r in reader.records() {
        let records = r.unwrap();
        let id = records.id().to_string();
        let seq = String::from_utf8(records.seq().to_vec()).unwrap();
        id2seq.insert(id, seq);
    }
    Ok(id2seq)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let map = read_fasta_core(path);
    println!("{:?}", map)
}