use std::collections::HashMap;

use crate::black_box::BlackBox;

pub struct BlackBoxKnowns{
    pub black_box: BlackBox,
    pub prefix_length: usize,
    pub secret_length: usize,
    pub block_length: usize,
    pub first_available_block_index: usize,
    pub first_available_block_padding: usize
}
impl BlackBoxKnowns {
    pub fn new(black_box: BlackBox) -> BlackBoxKnowns {
        let empty_input = black_box.encrypt_with_prefix(&mut test_input(0));
        let block_length = detect_block_length(&black_box);
        let prefix = detect_prefix_size(&black_box, block_length).unwrap();
        let zero_input_padding = detect_padding_size(&black_box).unwrap();
        
        println!("prefix length:{:?}", prefix);
        let secret_len =  empty_input.len() - (prefix + zero_input_padding);
        println!("secret length:{:?}", secret_len);

        let first_available_block:(usize, usize);
        if prefix < block_length {
            first_available_block = (block_length - prefix, 1);
        } else {
            first_available_block = (block_length - (prefix % block_length), (prefix / block_length) + 1 );
        }
        BlackBoxKnowns {
            black_box: black_box,
            block_length: block_length,
            prefix_length: prefix,
            secret_length: secret_len,
            first_available_block_padding: first_available_block.0,
            first_available_block_index: first_available_block.1,
            
        }
    }
}

// A hash of all possible encrypted blocks formed by changing the final byte on an input block
pub fn block_book(x:&Vec<u8>, knowns: &BlackBoxKnowns)-> HashMap<Vec<u8>, u8>{
    let mut out:HashMap<Vec<u8>, u8> = HashMap::new();
    for i in 0..255 {
        let char = i as u8;
        let mut byte = x.clone();
        byte.push(char);
        let mut input = vec![test_input(knowns.first_available_block_padding), byte].concat();
    
        let result = knowns.black_box.encrypt_with_prefix(&mut input);
        let block_start = knowns.first_available_block_index * knowns.block_length;
        let block_end = (knowns.first_available_block_index * knowns.block_length)+knowns.block_length;

        let relevant_block = result.get(block_start..block_end).unwrap().to_vec();

        out.insert(relevant_block, char);
    }
    out
}

pub fn block_map(knowns: &BlackBoxKnowns) -> HashMap<usize, (usize, usize)>{
    let mut ledger: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut found:Vec<usize> = Vec::new();
    let mut input_len = 0;
    while found.len() < knowns.secret_length {
        for block_i in 0..((knowns.prefix_length + input_len + knowns.secret_length)/knowns.block_length) {

            if block_i * knowns.block_length > knowns.prefix_length || knowns.prefix_length == 0 {
                let secret_index = ((block_i+1) * knowns.block_length) - (knowns.prefix_length + input_len)- 1;
                ledger.insert(secret_index, (input_len, block_i));
                found.push(secret_index);
            }
        }
        found.sort();
        found.dedup();

        // println!("{:?}/{:?} indecies found:{:?}", found.len(), secret_len, found);
        input_len += 1;   
    }
    ledger
}

pub fn detect_prefix_size(black_box: &BlackBox, block_size:usize) -> Option<usize>{
    let min_dups = duplicate_blocks(&black_box.encrypt_with_prefix(&mut test_input(0)), block_size);
    for i in 1..128 {
        let encrypted = black_box.encrypt_with_prefix(&mut test_input(i));
        let dup_count = duplicate_blocks(&encrypted, block_size);

        if dup_count > min_dups {
            let match_index = get_match_index(&encrypted, block_size).unwrap();
            return Some((block_size * match_index) - (i%block_size))
        }
    }
    None
}

pub fn detect_padding_size(black_box: &BlackBox) -> Option<usize>{
    let mut total_length:usize;
    let mut last_total_length = 0;
    for i in 0..128 {
        let encrypted = black_box.encrypt_with_prefix(&mut test_input(i));
        total_length = encrypted.len();
        if last_total_length != 0 &&  last_total_length != total_length {
            return Some(i);
        }
        last_total_length = total_length;
    }
    None
}

pub fn get_match_index(input: &Vec<u8>, block_size:usize) -> Option<usize> {
    let block_iter = (0..input.len()).step_by(block_size);
    for i in block_iter {
        let x = input.get(i..i+block_size);
        let y = input.get(i+block_size..i+(block_size*2));
        if x == y {
            return Some(i/block_size);
        }
    }
    None
}

pub fn duplicate_blocks(input: &Vec<u8>, block_size:usize) -> usize {
    let block_iter = (0..input.len()).step_by(block_size);
    let mut ledger: HashMap<Vec<u8>, usize> = HashMap::new();
    for i in block_iter {
        let block = input.get(i..i+block_size).unwrap().to_vec();
        ledger.entry(block)
        .and_modify(|e| { *e += 1 })
        .or_insert(1);
    }
    let mut keys: Vec<&usize> = ledger.values().collect();
    keys.sort_by(|a, b| b.cmp(a));
    *keys[0]

}

pub fn detect_block_length(black_box: &BlackBox) -> usize {
    let mut current_streak = 1;
    let mut scnd_streak = 0;
    let mut thrd_streak = 0;
    let mut last_len = 0;
    for i in 0..100{
        let bytes = black_box.encrypt_with_prefix(&mut test_input(i));

        if bytes.len() == last_len {
            current_streak += 1;
        }
        if last_len != 0 && bytes.len() != last_len {
            thrd_streak = scnd_streak;
            scnd_streak = current_streak;
            current_streak = 0;
        }
        if current_streak > 0 && thrd_streak > 0 && current_streak == scnd_streak {
            println!("block is {:?} long", current_streak + 1);
            return current_streak + 1;
        }

        last_len = bytes.len();
    }
    0
}

pub fn test_input(size:usize) -> Vec<u8>{
    let mut output: Vec<u8> = Vec::new();
    for _ in 0..size {
        output.push(b"X"[0])
    }
    output
}
