use std::collections::HashMap;


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
    // println!("bytes: {:?}", &encrypted);
    let mut ledger: HashMap<usize, Vec<usize>> = HashMap::new();
    // let encrypted = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    for i in 2..(encrypted.len()/4)+1 {
        println!("is it {:?} chars long?", i);
        let score;
        let ham = 0;
        
        let x = encrypted.get(0..i);
        let y = encrypted.get(i..(i*2));
        let a = encrypted.get(i*2..i*3);
        let b = encrypted.get(i*3..(i*4));

        // println!("x:{:?}", x);
        // println!("y:{:?}", y);
        // println!("a:{:?}", a);
        // println!("b:{:?}", b);

        
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
