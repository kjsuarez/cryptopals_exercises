use std::{collections::HashMap};
use openssl::symm::{Cipher, Mode, Crypter};
use rand::Rng;


// pub struct Encoder();
// impl Encoder {}

pub fn xor_encode(input: &[u8], key: &[u8]) -> Vec<u8>{
    let iter = 0..input.len();
    let mut output: Vec<u8> = Vec::new();
    for i in iter {
        let key_i = i % key.len();
        output.push(input[i] ^ key[key_i]);
    }
    output
}

pub fn hamming_distance(x: &[u8], y: &[u8]) -> usize {
    let iter = 0..x.len();
    let mut total = 0;
    for i in iter {
        total += (x[i] ^ y[i]).count_ones()
    }
    total as usize
}

pub fn guess_single_char_decode(encoded: &Vec<u8>) -> HashMap<usize, Vec<u8>> {
    let mut ledger: HashMap<usize, Vec<u8>> = HashMap::new();
    let byte_iter = (0 as u8)..(255 as u8);
    for key in byte_iter {
        let result= xor_encode(&encoded, &[key]);
        let score = human_readable_score(&result);

        if score > 0 {
            // ledger.insert(score, vec![String::from(output)]);
            ledger.entry(score)
            .and_modify(|e| { e.push(key) })
            .or_insert(vec![key]);
        }
    }
    ledger
}

pub fn human_readable_score(decrypted: &Vec<u8>) -> usize {
    let decoded_str = String::from_utf8(decrypted.clone());

    let output = match decoded_str {
        Ok(out) => out,
        Err(..) => String::new()
    };
    if output != "" {
        let x = &output.matches(|i: char| i.is_ascii_alphabetic() || i == ' ');
        let count: Vec<&str> = x.clone().collect();
        count.len()
    } else {
        return 0
    }
}

pub fn guess_key_length(encrypted: Vec<u8>) -> usize {
    let mut ledger: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 2..(encrypted.len()/2)+1 {
        let score;
        let ham;
        
        let x = encrypted.get(0..i);
        let y = encrypted.get(i..(i*2));
        
        ham = hamming_distance(x.unwrap(), y.unwrap());
        score = ham*1000 / i*1000;

        ledger.entry(score.clone())
        .and_modify(|e| { e.push(i.clone()) })
        .or_insert(vec![i.clone()]);
    }
    let mut keys: Vec<&usize> = ledger.keys().collect();
    keys.sort_by(|a, b| a.cmp(b));
    
    for el in keys {
        println!("{:?}: {:?}", el, &ledger[el])
    }
    0
}

pub fn guess_key_length_4_block_ham(encrypted: Vec<u8>) -> usize {
    let mut ledger: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 2..(encrypted.len()/4)+1 {
        println!("is it {:?} chars long?", i);
        let score;
        let ham = 0;
        
        let x = encrypted.get(0..i);
        let y = encrypted.get(i..(i*2));
        let a = encrypted.get(i*2..i*3);
        let b = encrypted.get(i*3..(i*4));
        
        let ham_1 = hamming_distance(x.unwrap(), y.unwrap());
        let ham_2 = hamming_distance(a.unwrap(), b.unwrap());
        score = ((ham_1*100 + ham_2*100)/2) / i;
        
          
        println!("score:{:?}, ({:?}/{:?})", score, ham, i);
        ledger.entry(score.clone())
        .and_modify(|e| { e.push(i.clone()) })
        .or_insert(vec![i.clone()]);
    
    }
    let mut keys: Vec<&usize> = ledger.keys().collect();
    keys.sort_by(|a, b| a.cmp(b));
    
    for el in keys {
        println!("{:?}: {:?}", el, &ledger[el])
    }
    0
}

pub fn guess_key_of_size(encrypted: Vec<u8>, key_size: usize) -> Vec<Vec<u8>>{
    // for each block of key_size bytes
    let block_iter = (0..encrypted.len()).step_by(key_size);
    let mut pos_blocks: Vec<Vec<u8>> = Vec::new();
    for _i in 0..key_size {
        pos_blocks.push(Vec::new());
    }

    // build pos_blocks, a vec of vecs, 1 for each char in the key
    for i in block_iter {
        for j in 0..key_size {
            let val = encrypted.get(i+j);
            match val {
                None => (),
                Some(thing) => {
                    pos_blocks[j].push(*thing);
                },
            }
        }
    }
    let mut possible_keys:Vec<Vec<u8>> = Vec::new();
    for block in pos_blocks {
        let ledger = guess_single_char_decode(&block);
        let mut keys: Vec<&usize> = ledger.keys().collect();
        keys.sort_by(|a, b| b.cmp(a));
        for _el in &keys {
        }
        possible_keys.push(ledger[keys[0]].clone());
    }
    
    possible_keys
}

pub fn ebc_block_decrypt(block: &[u8; 16], key: &[u8]) -> [u8; 16] {
    let mut encrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None
    ).unwrap();
    encrypter.pad(false);
    // I don't know why I need a 32 byte buffer 
    // for a 16 byte block but whatever
    let mut x: [u8; 32] = [0; 32];
    let _ = encrypter.update(block, &mut x);
    let decrypted_block = x.get(0..16).unwrap();

    <[u8; 16]>::try_from(decrypted_block).unwrap()
}

pub fn ebc_block_encrypt(block: &[u8; 16], key: &[u8]) -> [u8; 16] {
    let mut encrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Encrypt,
        key,
        None
    ).unwrap();
    encrypter.pad(false);
    // I don't know why I need a 32 byte buffer 
    // for a 16 byte block but whatever
    let mut x: [u8; 32] = [0; 32];
    let _ = encrypter.update(block, &mut x);
    let encrypted_block = x.get(0..16).unwrap();

    <[u8; 16]>::try_from(encrypted_block).unwrap()
}

pub fn cbc_block_decrypt(block: &[u8; 16], last_block: [u8; 16], key: &[u8]) -> [u8; 16] {
    let mut output: [u8;16] = [0; 16];
    let decrypted_block = ebc_block_decrypt(&block, key);
    for i in 0..16 {
        output[i] = decrypted_block[i] ^ last_block[i];
    }
    output
}

pub fn cbc_block_encrypt(block: &[u8; 16], last_block: [u8; 16], key: &[u8]) -> [u8; 16] {
    let mut xord_block = block.clone();
    for i in 0..16 {
        xord_block[i] = block[i] ^ last_block[i];
    }
    ebc_block_encrypt(&xord_block, key)
}

pub fn cbc_ecrypt(plain_text:&[u8], key: &[u8]) -> Vec<u8> {
    let iv: &[u8; 16] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    let mut output:Vec<u8> = Vec::new();
    let encode_iter = (0..plain_text.len()).step_by(16);
    for i in encode_iter {
        let block = <[u8; 16]>::try_from(plain_text.get(i..i+16).unwrap()).unwrap();
        let mut last_block = *iv;
        if i != 0 {
            last_block = <[u8; 16]>::try_from(output.get(i-16..i).unwrap()).unwrap();
        }
  
        let encrypted_block = cbc_block_encrypt(&block, last_block, key);
        for byte in encrypted_block {
            output.push(byte);
        }
    }
    output
}

pub fn cbc_decrypt(encoded:&[u8], key: &[u8]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let decoder_iter = (0..encoded.len()).step_by(16);
    let iv: &[u8; 16] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    
    for i in decoder_iter {
        let mut last_block = *iv;
        if i != 0 {
            last_block = <[u8; 16]>::try_from(encoded.get(i-16..i).unwrap()).unwrap();
        }
        let block = &<[u8; 16]>::try_from(encoded.get(i..i+16).unwrap()).unwrap();
        let decoded_block = cbc_block_decrypt(block, last_block, key);
        for byte in decoded_block{
            output.push(byte);
        }
    }
    output
}

pub fn ebc_encrypt(plain_text:&[u8], key: &[u8]) -> Vec<u8> {
    if plain_text.len() % 16 != 0 {
        panic!()
    }
    let mut output:Vec<u8> = Vec::new();
    let encode_iter = (0..plain_text.len()).step_by(16);
    for i in encode_iter {
        let block = <[u8; 16]>::try_from(plain_text.get(i..i+16).unwrap()).unwrap();
        let encrypted_block = ebc_block_encrypt(&block, key);
        for byte in encrypted_block {
            output.push(byte);
        } 
    }
    output
}

pub fn ebc_decrypt(encrypted:&[u8], key: &[u8]) -> Vec<u8> {
    println!("length of vec to decrypt:{:?}", encrypted.len());
    let mut output:Vec<u8> = Vec::new();
    let encode_iter = (0..encrypted.len()).step_by(16);
    for i in encode_iter {
        let block = <[u8; 16]>::try_from(encrypted.get(i..i+16).unwrap()).unwrap();
        let plain_block = ebc_block_decrypt(&block, key);
        for byte in plain_block {
            output.push(byte);
        } 
    }
    output
}

pub fn random_aes_key() -> [u8;16] {
    let mut rng = rand::thread_rng();
    let mut output = [0;16];
    for i in 0..16 {
       output[i] = rng.gen::<u8>() 
    }
    output
}