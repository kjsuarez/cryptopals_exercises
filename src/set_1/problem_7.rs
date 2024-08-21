use openssl::symm::{decrypt, Cipher};
use std::fs::File;
use std::io::Read;
use base64::prelude::*;

pub fn main(){
    let cipher = Cipher::aes_128_ecb();
    let _plain_text = "This is a test".as_bytes();
    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    
    // encrypt example

    // let ciphertext = encrypt(
    //     cipher,
    //     key,
    //     Some(iv),
    //     plain_text
    // ).unwrap();
    // println!("encrypted: {:?}", &ciphertext);

    
    // get encoded bytes from b64
    let mut file = File::open("src/set_1/problem_7_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.replace("\n", "");
    let encoded = BASE64_STANDARD.decode(contents.as_bytes()).unwrap();

    // decrypt
    let ciphertext = decrypt(
        cipher,
        key,
        Some(iv),
        &encoded
    ).unwrap();
    println!("decrypted: {:?}", String::from_utf8(ciphertext));
}