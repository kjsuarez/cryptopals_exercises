use crate::encoder;
use crate::byte_tools::ByteString;

pub fn main() {
    
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key: &str = "ICE";
    let input_bytes = input.as_bytes();
    let key_bytes = key.as_bytes();
    let encoded_bytes = encoder::xor_encode(input_bytes, key_bytes);
    println!("{:x?}", &encoded_bytes);

    let real = ByteString::from_hex_str("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").bytes.unwrap();
    assert_eq!(&encoded_bytes, &real);

    let decoded_bytes = encoder::xor_encode(&real, key_bytes);
    println!("the real deal: {:?}", String::from_utf8(decoded_bytes));
}