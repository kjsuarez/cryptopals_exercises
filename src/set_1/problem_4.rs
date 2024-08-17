use std::collections::HashMap;

use crate::byte_tools::ByteString;
use std::fs::File;
use std::io::Read;

pub fn main() {


    // hash practice
    let mut results: HashMap<usize, Vec<String>> = HashMap::new();


    // file reading, iterating
    let mut file = File::open("src/set_1/problem_4_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines_iter = contents.lines();
    for line in lines_iter {
        println!("{:?}", line);
        let bytes = ByteString::from_hex_str(&(line.replace("\n", ""))).bytes.unwrap();
        record_decode(&bytes, &mut results);
    }
    let mut keys: Vec<&usize> = results.keys().collect();
    keys.sort();
    println!("WINNER: {:?}", results[keys.last().unwrap()]);
}

fn human_score(decoded:&str) -> usize {
    let count: Vec<&str> = decoded.matches(" ").collect();
    count.len()
}


fn record_decode(encoded: &Vec<u8>, ledger: &mut HashMap<usize, Vec<String>>) {
    let byte_iter = (0 as u8)..(255 as u8);
    for key in byte_iter {
        let result= String::from_utf8(xor_decode(&encoded, key));
        let output = match result {
            Ok(out) => out,
            Err(..) => String::new()
        };
        if output != "" {
            let score = human_score(&output);
            if score > 0 {
                // ledger.insert(score, vec![String::from(output)]);
                ledger.entry(score)
                .and_modify(|e| { e.push(output.clone()) })
                .or_insert(vec![output.clone()]);
            }
        }
    }

}

fn xor_decode(encoded: &Vec<u8>, key: u8) -> Vec<u8> {
    let iter = 0..encoded.len();
    iter.map(|i|
        encoded[i] ^ key
    ).collect()
}
