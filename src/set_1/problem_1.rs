use std::str;
use crate::byte_tools::ByteString;



pub fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    // let input = "49276d";
    
    let b_str = ByteString::from_hex_str(&input);

    let decoded = b_str.b64_string();
    println!("decoded:{:?}", decoded);
    assert_eq!(decoded.unwrap(), String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
}


fn notes() {
    let n = 0x6d;
    let bytes_by_hand: [u8; 1] = [n];
    match str::from_utf8(&bytes_by_hand) {
        Ok(my_str) => println!("by handsuccess: '{}'", my_str),
        Err(e) => println!("by hand failed: {:?}", e),
    };
    
    let bytes1: &[u8; 3] = b"\x49\x27\x6d";
    let bytes2: &[u8; 6] = b"49276d";
    println!("A byte string: {:?}", bytes1);
    match str::from_utf8(bytes1) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };

    match str::from_utf8(bytes2) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };

    // prove that you can turn a string into a byte
    let char = "6d";
    let byte_from_char = u8::from_str_radix(char, 16).ok();
    let proof_array: [u8; 1] = [byte_from_char.unwrap()];
    match str::from_utf8(&proof_array) {
        Ok(my_str) => println!("proof successful: '{}'", my_str),
        Err(e) => println!("proof failed: {:?}", e),
    };


    // Prove that you can index an array with a byte 
    let index_byte = u8::from_str_radix("00000001", 2).ok().unwrap();
    let ary = ["one", "two", "three"];
    println!("array at index 1: {:?}", ary[1]);
    println!("array at index 00000001: {:?}", ary[index_byte as usize]);

    // Prove that you can shift a byte left and right
    let some_byte = u8::from_str_radix("10101011", 2).ok().unwrap();
    println!("\nthe original byte: {:08b}", some_byte);
    
    let split_byte = first_6_split(some_byte);
    println!("split left: {:08b}, {:08b}", split_byte.0, split_byte.1);

    let snd_split = last_6_split(some_byte);
    println!("split right: {:08b}, {:08b}", snd_split.0, snd_split.1);

    let shifted = split_byte.1 << 6u8;
    println!("\ntwo bits shifted left: {:08b}", shifted);

    println!("\naddition: {:08b}", snd_split.0 + split_byte.1);
    
}

fn first_6_split(byte: u8) -> (u8, u8) {
    let left_6: u8 =  byte & 0b11111100;
    let right_2: u8 = byte & 0b00000011;
    return (left_6, right_2)
}

fn last_6_split(byte: u8) -> (u8,u8) {
    let left_2: u8 =  byte & 0b11000000;
    let right_6: u8 = byte & 0b00111111;
    return (left_2, right_6)
}

