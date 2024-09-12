use std::collections::HashMap;
use std::hint::black_box;

use base64::prelude::*;

use rand::Rng;
use crate::{black_box, encoder, pkcs7};


use crate::black_box::{BlackBox};

pub fn main() {
    let (secret, black_box, iv) = random_encrypted();
    let mut breaker = CbcBreaker::new(iv, secret, &black_box);
    breaker.build_keystream();
}

struct CbcBreaker {
    black_box: BlackBox,
    keystream: Vec<u8>,
    working_keystream: Vec<u8>,
    iv: Vec<u8>,
    working_iv: Vec<u8>,
    complete: Vec<u8>,
    working: Vec<u8>,
    secret: Vec<u8>,
    plaintext: Vec<u8>,
}

impl CbcBreaker {
    const BLOCKSIZE:usize = 16;
    fn new(iv: Vec<u8>, secret: Vec<u8>, black_box: &BlackBox) -> CbcBreaker{
        let mut whole_hog = iv.clone();
        
        whole_hog.append(&mut secret.clone());
        CbcBreaker {
            iv: iv.clone(),
            secret: secret.clone(),
            complete: whole_hog.clone(),
            working: secret,
            working_iv: iv,
            working_keystream: Vec::new(),
            black_box: black_box.clone(),
            keystream: Vec::new(),
            plaintext: Vec::new(),
        }
    }

    fn set_plaintext(&mut self) {
        let complete_len = self.complete.len();
        let keystream_len = self.keystream.len();
        let relevant = self.complete.get(0..complete_len-Self::BLOCKSIZE).unwrap();
        
        self.keystream.reverse();
        println!("{:?}", self.keystream);
        for i in 0..relevant.len() {
            self.plaintext.push(relevant[i] ^ self.keystream[i]);
        }
        println!("plaintext: {:?}", String::from_utf8_lossy(&self.plaintext));
    }

    fn get_last_keystream(secret: &Vec<u8>, black_box: &BlackBox, iv: &Vec<u8>) -> Vec<u8> {   
        let mut rng = rand::thread_rng();
        let mut good_bytes: Vec<u8> = Vec::new();
        let mut probe_secret = secret.clone();
        let length = probe_secret.len();
        if length > Self::BLOCKSIZE {
            let scnd_to_last_index_of_scnd_to_last_block = length - (Self::BLOCKSIZE+2);
            let last_index_of_scnd_to_last_block = length - (Self::BLOCKSIZE+1);
            println!("setting index {:?} to 0", scnd_to_last_index_of_scnd_to_last_block);
            probe_secret[scnd_to_last_index_of_scnd_to_last_block] = rng.gen_range(0..255);
            println!("geting last keystream byte, changing byte at index {:?}", last_index_of_scnd_to_last_block);
            for i in (0..255) {
                probe_secret[last_index_of_scnd_to_last_block] = i;
                if is_valid_padding(&mut probe_secret, &black_box, iv) {
                    good_bytes.push(i^1);
                }
            }
            if good_bytes.len() != 1 {
                
                
                println!("trying again!!!");
                return Self::get_last_keystream(secret, black_box, iv);
                
                // println!("ERROR found {:?} viable bytes", good_bytes.len());
                
                // panic!()
            }
        } else {
            let mut probe_iv = iv.clone();
            probe_iv[Self::BLOCKSIZE - 2] = 255;
            for i in (0..255) {
                probe_iv[Self::BLOCKSIZE - 1] = i;
                if is_valid_padding(&mut probe_secret, &black_box, &probe_iv) {
                    good_bytes.push(i^1);
                }
            }
            if good_bytes.len() != 1 {
                println!("trying again!!!");
                return Self::get_last_keystream(secret, black_box, iv);
                // println!("ERROR found {:?} viable bytes", good_bytes.len());
                // panic!()
            }
        }
 
        good_bytes
    }

    fn build_keystream_block(&mut self) {
        let mut first_keystream = Self::get_last_keystream(&self.working, &self.black_box, &self.iv);
        self.working_keystream.append(&mut first_keystream);
        println!("working keystream:{:?}", self.working_keystream);
        println!("working length:{:?}", self.working.len());
        let upper_bound: usize;
        let x: usize;
        if self.working_keystream.len() == 1 {
            upper_bound = Self::BLOCKSIZE -1;
            x = 1;
        } else {
            upper_bound = Self::BLOCKSIZE;
            x = 0;
        }

        for i in 0..upper_bound {
            let mut good_bytes: Vec<u8> = Vec::new();
            let edit_i = self.working.len() - (1 + x + i); //change_i+16;
            let change_i = edit_i - Self::BLOCKSIZE;
            println!("changing {:?} to edit {:?}", change_i, edit_i, );
            println!("About to set padding bytes, known keystream char count: {:?}", self.working_keystream.len() );
            self.set_padding_bytes();

            // for 0..255  
            //      when you find a char that passes padding, add it to array
            // raise if passing array length is more than 1
            for j in 0..255 {
                self.working[change_i] = j;
                if is_valid_padding(&mut self.working, &self.black_box, &self.iv) {
                    // println!("HERE - Adding ({:?} ^ {:?}) to keystream", j, self.working_keystream.len()+1);
                    good_bytes.push(j^((self.working_keystream.len()+1) as u8));
                }
            }
            if good_bytes.len() != 1 {
                println!("Wrong number of viable bytes: {:?}", good_bytes.len());
                panic!()
            }
            self.working_keystream.append(&mut good_bytes);
            println!("keystream:{:?}", self.working_keystream);
        }

        self.working.truncate(self.working.len()- Self::BLOCKSIZE);
        self.keystream.append(&mut self.working_keystream);
        self.working = self.secret.get(0..self.working.len()).unwrap().to_vec();
        self.working_keystream.clear();
        println!("new working secret:{:?}", self.working);
        println!("new working keystream:{:?}", self.working_keystream);
    }

    fn build_first_keystream_block(&mut self) {
        let mut first_keystream = Self::get_last_keystream(&self.working, &self.black_box, &self.iv);
        self.working_keystream.append(&mut first_keystream);

        let upper_bound: usize;
        let x: usize;
        if self.working_keystream.len() == 1 {
            upper_bound = Self::BLOCKSIZE -1;
            x = 1;
        } else {
            upper_bound = Self::BLOCKSIZE;
            x = 0;
        }
        println!("\nLAST BLOCK");
        println!("Starting at index {:?}", upper_bound);
        for i in 0..upper_bound {
            let search_index = upper_bound-(i+1);
            println!("looking for keystream at index {:?} of working iv", search_index);
            let mut good_bytes: Vec<u8> = Vec::new();
            
            self.set_padding_bytes();
            
            for j in 0..255 {
                self.working_iv[search_index] = j;
                if is_valid_padding(&mut self.working, &self.black_box, &self.working_iv) {
                    // println!("HERE - Adding ({:?} ^ {:?}) to keystream", j, self.working_keystream.len()+1);
                    good_bytes.push(j^((self.working_keystream.len()+1) as u8));
                }
            }
            if good_bytes.len() != 1 {
                println!("Wrong number of viable bytes: {:?}", good_bytes.len());
                panic!()
            }
            self.working_keystream.append(&mut good_bytes);
        }

        self.working.truncate(self.working.len()- Self::BLOCKSIZE);
        self.keystream.append(&mut self.working_keystream);
        self.working_keystream.clear();
        println!("new working length:{:?}", self.working.len());
    }

    

    fn build_keystream(&mut self) {
        while self.working.len() > 0 {
            if self.working.len() == Self::BLOCKSIZE {
                // we build the 1st keystream block last
                self.build_first_keystream_block();
            } else {
                println!("\nBuilding keystream block!");
                println!("working secret: {:?}", self.working);
                self.build_keystream_block();
            }
        }
        println!("full keystream: {:?}", self.keystream);
        self.set_plaintext();
    }

    fn set_padding_bytes(&mut self) {
        let intended_val = self.working_keystream.len() + 1;
        // println!("setting the last {:?} bytes of secret to {:?}", self.working_keystream.len(), intended_val);
        for i in 0..self.working_keystream.len() {
            let encrypted_padding = self.working_keystream[i] ^ (intended_val as u8);
            // println!("{:?} ^ {:?} = {:?}", self.working_keystream[i], intended_val, encrypted_padding);
            let length = self.working.len();
            if self.working.len() <= Self::BLOCKSIZE {                
                self.working_iv[length-(1+i)] = encrypted_padding;
            } else {
                self.working[length-(1+i + Self::BLOCKSIZE)] = encrypted_padding;             
            }
        }
    }
}



fn random_secret() -> Vec<u8> {
    let choices = [
        "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
        "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
        "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
        "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
        "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
        "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
        "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
        "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
        "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
        "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"
    ];
    let mut rng = rand::thread_rng();
    let mut picked = choices[rng.gen_range(0..choices.len()-1)];

    picked = "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl";

    BASE64_STANDARD.decode(picked).unwrap()
}

fn random_encrypted() -> (Vec<u8>, BlackBox, Vec<u8>){
    let black_box = BlackBox::new();
    let iv=  encoder::random_aes_key().to_vec();//b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec();
    let encrypted = black_box.cbc_encrypt_with_iv(&iv,&mut random_secret());
    (encrypted, black_box, iv)
}

fn is_valid_padding(input: &mut Vec<u8>, black_box: &BlackBox, iv: &Vec<u8>) -> bool{
    let decrypted = black_box.cbc_decrypt_keep_padding(input, iv);
    let padding_check = pkcs7::strip(decrypted.clone());
    match padding_check {
        Ok(_) => {
            println!("{:?}", String::from_utf8_lossy(&decrypted));
            true
        },
        Err(_) => false,
    }
}