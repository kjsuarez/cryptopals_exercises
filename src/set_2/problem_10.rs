use std::fs::File;
use std::io::Read;
use base64::prelude::*;

use crate::encoder;

pub fn main(){
    let key = b"YELLOW SUBMARINE";
    let mut file = File::open("src/set_2/problem_10_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.replace("\n", "");
    let encoded = BASE64_STANDARD.decode(contents.as_bytes()).unwrap();

    let decoded = encoder::cbc_decrypt(&encoded, key);
    println!("{:?}", String::from_utf8(decoded));
}


