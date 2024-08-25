use base64::prelude::*;

use crate::encoder;
use std::{collections::HashMap};

// The point (I think) is to demonstrate that you can break 
// a key if you have controle of the input?

pub fn main(){
    // I won't use this key to decrypt, scout's honor
    let key: [u8;16] = [25, 130, 156, 43, 47, 237, 129, 14, 162, 77, 206, 90, 44, 126, 247, 242];
    let secret_str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let secret_input = BASE64_STANDARD.decode(secret_str.as_bytes()).unwrap();
    
    println!("blocks are {:?} bytes long", determine_block_size(&secret_input, &key));

    let x = decrypt(&secret_input, &key, 16);
    println!("{:?}", String::from_utf8(x));
}

fn test_input(size:usize) -> Vec<u8>{
    let mut output: Vec<u8> = Vec::new();
    for _ in 0..size {
        output.push(b"X"[0])
    }
    output
}

fn decrypt(secret_input: &Vec<u8>, key:&[u8], block_size: usize) -> Vec<u8> {
    let mut output = Vec::new();
    let mut dictionary: HashMap<Vec<u8>, u8> = HashMap::new();
    for i in 0..255 {
        let char = i as u8;
        let mut x = test_input(block_size - 1);
        x.push(char.clone());
        let result = mystery_encryption(&mut x, &key);
        let result_block = result.get(0..block_size).unwrap().to_vec();
        dictionary.insert(result_block, i);
    }
    let mut input:Vec<u8>;
    for i in 0..secret_input.len() {
        input = test_input(block_size - 1);
        input.push(secret_input[i]);
        let encrypted = mystery_encryption(&mut input, &key);
        let char = dictionary[&encrypted];
        println!("{:?}", char);
        output.push(char);
    }

    output
}

fn mystery_encryption(input: &mut Vec<u8>, key: &[u8]) -> Vec<u8> {
    encoder::pkcs7(input, 16);
    encoder::ebc_encrypt(&input, key)
}

fn determine_block_size(secret_input:&Vec<u8>, key:&[u8]) -> usize{
    let mut test_input:Vec<u8> = "X".as_bytes().iter().cloned().collect();
    let mut x = test_input.clone();
    x.append(&mut secret_input.clone());
    let encrypted = mystery_encryption(&mut x, &key);
    let mut last_block = encrypted.get(0..test_input.len()).unwrap().to_vec();


    for _ in 0..128 {
        test_input.push(b"X"[0]);
        let mut x = test_input.clone();
        x.append(&mut secret_input.clone());
        let encrypted = mystery_encryption(&mut x, &key);
        let possible_block = encrypted.get(0..test_input.len()).unwrap().to_vec();

        if last_block.get(..).unwrap() == possible_block.get(0..(possible_block.len()-1)).unwrap() {
            
            return test_input.len() - 1
        }
        last_block = possible_block.clone();
        
    }
    0
}