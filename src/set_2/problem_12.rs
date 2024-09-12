use crate::black_box::{BlackBox};
use crate::ecb_tools::{*, BlackBoxKnowns};


// The point (I think) is to demonstrate that you can break 
// a key if you have controle of the input?

pub fn main(){
    let black_box = BlackBox::new();
    let knowns = BlackBoxKnowns::new(black_box);
    
    // key: index of secret character (not including prefix or input)
    // value: (user input length, block index) combination required to isolate that secret index at the end of a block
    let phone_book = block_map(&knowns);
    
    let mut decoded:Vec<u8> = Vec::new();
    // block_length-1 vec of characters used to decrypt the secret input
    let mut block_input = test_input(knowns.block_length-1);

    for i in 0..knowns.secret_length {
        let (input_len, block_i) = phone_book[&i];
        let encrypted = knowns.black_box.encrypt_with_prefix(&mut test_input(input_len));
        let block_index_start = block_i * knowns.block_length;
        let block_index_end = (block_i*knowns.block_length)+knowns.block_length;
        let block = encrypted.get(block_index_start..block_index_end).unwrap();
        
        // every possible block created by adding some char to block input
        let possibilities = block_book(&block_input, &knowns);

        // plug in the real block and reveal the char at position i
        let char = possibilities.get(block);
        match char {
            Some(char) => {
                // add it to decoded
                decoded.push(*char);
                        
                // take off the first element of block_input append the char
                // we're just moving a block frame across the secret one character at a time
                block_input.remove(0);
                block_input.push(*char);
            },
            None => {
                println!("can't find block in the possible set. Either something went wrong or you hit padding bytes");
                println!("x{:?}",String::from_utf8(decoded.clone()));
                break;
            }
        }
    }
    println!("{:?}",String::from_utf8(decoded));
}

