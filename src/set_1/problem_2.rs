use crate::byte_tools::ByteString;

pub fn main() {
    let input_1: &str = "1c0111001f010100061a024b53535009181c";
    let input_2: &str = "686974207468652062756c6c277320657965";
    let bytes1: ByteString = ByteString::from_hex_str(input_1);
    let bytes2: ByteString = ByteString::from_hex_str(input_2);
    println!("xored bytes: {:x?}", bytes1.xor(bytes2).unwrap());
    
    // prove you know how to xor
    println!("Input bytes: {:x?}", bytes1.bytes.unwrap());
    println!("xored: {:x?}", b"\x1C"[0] ^ b"\x68"[0]);

}
