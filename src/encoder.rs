



// pub struct Encoder();

// impl Encoder {
    
// }

pub fn xor_encode(input: &[u8], key: &[u8]) -> Vec<u8>{
    let iter = 0..input.len();
    let mut output: Vec<u8> = Vec::new();
    for i in iter {
        let key_i = i % key.len();
        output.push(input[i] ^ key[key_i]);
    }
    output
}