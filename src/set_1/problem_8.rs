use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;
use crate::byte_tools::ByteString;

pub fn main(){
    // The key is that you can see the 16byte blocks.
    // Count the recurances of each 16 byte block and pick 
    // the string with the highest recurrence.

    let mut file = File::open("src/set_1/problem_8_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines_iter = contents.lines();
    let mut largest: (usize, &str) = (0,"");
    for line in lines_iter {
        let bytes = ByteString::from_hex_str(&(line.replace("\n", ""))).bytes.unwrap();
        let recurrences = count_recurrences(bytes);
        if recurrences > largest.0 {
            largest = (recurrences, line);
        }
    }
    println!("I think it's {:?}", largest);
}

fn count_recurrences(encrypted: Vec<u8>) -> usize {
    let mut ledger: HashMap<&[u8], usize> = HashMap::new();
    let byte_iter = (0..encrypted.len()).step_by(16);
    for i in byte_iter {

        let x = encrypted.get(i..i+16).unwrap();
        
        ledger.entry(x)
        .and_modify(|num| { *num += 1 })
        .or_insert(1);

    }

    let mut counts: Vec<&usize> = ledger.values().collect();
    counts.sort_by(|a, b| b.cmp(a));
    *counts[0]
}