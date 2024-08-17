use crate::byte_tools::ByteString;

pub fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = ByteString::from_hex_str(input).bytes.unwrap();
    println!("bytes: {:x?}", bytes);
    println!("decoded: {:?}", String::from_utf8(bytes.clone()).unwrap());
    
    let byte_iter = (0 as u8)..(255 as u8);
    for key in byte_iter {
        let result= String::from_utf8(xor_decode(&bytes, key));
        let output = match result {
            Ok(out) => out,
            Err(..) => String::new()
        };
        if output != "" {
            println!("\nkey:{:x?} decoded: {:?}", key, output);
        }
    }
}

fn xor_decode(encoded: &Vec<u8>, key: u8) -> Vec<u8> {
    let iter = 0..encoded.len();
    iter.map(|i|
        encoded[i] ^ key
    ).collect()
}