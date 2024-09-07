
pub fn pkcs7(input: &mut Vec<u8>, block_size: usize) {
    let pad_size: usize = block_size - (input.len() % block_size);
    for _ in 0..pad_size {
        input.push(pad_size as u8);
    }
}

pub fn strip_pkcs7(input: &mut Vec<u8>) {
    for _ in 0..*input.last().unwrap() {
        input.pop();
    }
}

pub fn strip(mut input: Vec<u8>) -> Result<Vec<u8>, String>{
    if input.len() % 16 != 0 {
        return Err("Input length is not a multiple of 16".to_string());
    }
    
    let padding_byte = *input.last().unwrap();
    for _ in 0..padding_byte {
        if input.last().unwrap() == &padding_byte {
            input.pop();
        } else {
            return Err("Incorrect number of padding bytes".to_string());
        }
    }
    Ok(input)
}