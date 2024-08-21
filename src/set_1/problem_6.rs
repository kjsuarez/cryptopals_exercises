
use base64::prelude::*;
use std::fs::File;
use std::io::Read;



use crate::encoder;



pub fn main() {
    // get encoded bytes
    let mut file = File::open("src/set_1/problem_6_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.replace("\n", "");

    let encoded = BASE64_STANDARD.decode(contents.as_bytes()).unwrap();
    
    // guess the length of the key
    let key_length_guess = encoder::guess_key_length(encoded.clone());
    println!("Key length gueses: {:?}", key_length_guess);
    
    // try to guess key based on length
    let guess = encoder::guess_key_of_size(encoded.clone(), 29);
    for i in 0..guess.len() {
        println!("key char {:?}: {:?}", i, String::from_utf8(guess[i].clone()))
    }

    // print decoded string
    let decoded = encoder::xor_encode(&encoded, b"Terminator X: Bring the noise");
    println!("{:?}", String::from_utf8(decoded));


    // let x = b"this is a test";
    // let y = b"wokka wokka!!!";
    // let ham = encoder::hamming_distance(x, y);
    // println!("Hamming distance is {:?}", ham);
}

