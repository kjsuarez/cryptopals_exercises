use crate::encoder;
pub fn main(){
    // The rules for PKCS padding are very simple:
    // - Padding bytes are always added to the clear text before it is encrypted.
    // - Each padding byte has a value equal to the total number of padding bytes that are added. For example, if 6 padding bytes must be added, each of those bytes will have the value 0x06.
    // - The total number of padding bytes is at least one, and is the number that is required in order to bring the data length up to a multiple of the cipher algorithm block size.

    let mut test = b"YELLOW SUBMARINE".to_vec();
    encoder::pkcs7(&mut test, 20);
    assert_eq!("YELLOW SUBMARINE\x04\x04\x04\x04", String::from_utf8(test).unwrap());

    let mut test1 = b"YELLOW SUBMARINE".to_vec();
    encoder::pkcs7(&mut test1, 16);
    assert_eq!("YELLOW SUBMARINE", String::from_utf8(test1).unwrap());

    let mut test2 = b"YELLOW SUBMARINEEE".to_vec();
    encoder::pkcs7(&mut test2, 16);
    assert_eq!("YELLOW SUBMARINEEE\x0e\x0e\x0e\x0e\x0e\x0e\x0e\x0e\x0e\x0e\x0e\x0e\x0e\x0e", String::from_utf8(test2).unwrap());
}

