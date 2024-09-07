use crate::encoder;
use crate::pkcs7::*;
use std::collections::HashMap;
use rand::Rng;

// This one really threw me for a loop. 
// The prompt is very specific about everything except the plaintext input
// At first I thought I was solving for *any* plaintext input of any size
// but I couldn't find a reliable way to determine ecb vs cbc 
// besides the one already covered in problem 8. Looking for block patterns is 
// pointless on any input that's too short or generally doesn't have repeating blocks.
// Online discussions of the problem interpret it as 
// "Given random padding, IV, and key, accurately determine encryption used
// *some input* of your choise." I'm not sattisfied with this conclusion but 
// I don't really know how else to proceed.

pub fn main() {
    let input: Vec<u8> = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".as_bytes().iter().cloned().collect();
    
    for _ in 0..100{
        let mystery = ecb_or_cbc_encrypt(&mut input.clone());
        print!(".");
        assert_eq!(guess_encryption(&mystery.0), mystery.1);
        println!("{:?}->{:?}", guess_encryption(&mystery.0), &mystery.1)
    }

}

fn guess_encryption(input: &Vec<u8>) -> String {
    let recurrence = count_recurrences(input);
    if recurrence > 1 {
        String::from("ecb")
    } else {
        String::from("cbc")
    }
}

fn ecb_or_cbc_encrypt(plain_text:&mut Vec<u8>) -> (Vec<u8>, String) {
    let padded_input = padded_plaintext(plain_text);
    let mut rng = rand::thread_rng();
    let coin:usize = rng.gen_range(0..2);

    match coin {
        0 => {
            let ecb_encoded = encoder::ebc_encrypt(&padded_input, &random_aes_key());
            (ecb_encoded, String::from("ecb"))
        },
        1 => {
            let cbc_encoded = encoder::cbc_ecrypt(&padded_input, &random_aes_key());
            (cbc_encoded, String::from("cbc"))
        },
        _=> (Vec::new(), String::from("fuck"))
    }
}

fn count_recurrences(encrypted: &Vec<u8>) -> usize {
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

fn random_aes_key() -> [u8;16] {
    let mut rng = rand::thread_rng();
    let mut output = [0;16];
    for i in 0..16 {
       output[i] = rng.gen::<u8>() 
    }
    output
}

fn padded_plaintext(input: &mut Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut prefix: Vec<u8> = random_bytes();
    let mut suffix: Vec<u8> = random_bytes();
    output.append(&mut prefix);
    output.append(input);
    output.append(&mut suffix);
    pkcs7(&mut output, 16);
    output
}

fn random_bytes() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut output:Vec<u8> = Vec::new();
    for _ in 0..rng.gen_range(5..10) {
        output.push(rng.gen::<u8>());
    }
    output
}