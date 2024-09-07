use crate::pkcs7;

pub fn main() {
    let err_1 = "Input length is not a multiple of 16".to_string();
    let err_2 = "Incorrect number of padding bytes".to_string();
    
    let bad_block_len:Vec<u8> = "Isn't a factor of 16".as_bytes().to_vec();
    assert_eq!(pkcs7::strip(bad_block_len), Err(err_1));

    let bad_padding_len:Vec<u8> = "Is a factor of 16 but padding is busted\x09\x09\x09\x08\x09\x09\x09\x09\x09".as_bytes().to_vec();
    assert_eq!(pkcs7::strip(bad_padding_len), Err(err_2));

    let properly_padded:Vec<u8> = "0123456789abcde\x01".as_bytes().to_vec();
    assert_eq!(pkcs7::strip(properly_padded), Ok("0123456789abcde".as_bytes().to_vec()));

    let properly_padded_2:Vec<u8> = "0123456789abcd\x02\x02".as_bytes().to_vec();
    assert_eq!(pkcs7::strip(properly_padded_2), Ok("0123456789abcd".as_bytes().to_vec()));
}