
pub struct ByteString {
    pub hex_string: String,
    pub bytes: Option<Vec<u8>>,
    b64_bytes: Option<Vec<u8>>,
    pub b64_string: Option<String>,
}

impl ByteString {
    pub fn from_hex_str(hex_str: &str) -> ByteString{
        let bytes = ByteString::hex_str_to_bytes(hex_str);
        let b64_bytes: Option<Vec<u8>> = match &bytes {
            Some(b) => ByteString::bytes_to_b64(b),
            None => None,
        };
        let b64_string:Option<String> = match &b64_bytes {
            Some(b) => Some(ByteString::b64_to_string(b)),
            None => None,
        };

        ByteString {
            hex_string: String::from(hex_str),
            bytes: bytes,
            b64_bytes: b64_bytes,
            b64_string: b64_string,
        }
    }

    fn hex_str_to_bytes(input: &str) -> Option<Vec<u8>> {
        if input.len() % 2 > 0 {
            return None
        } else {
            let hex_iter = (0..input.len()).step_by(2);
            let thing:Vec<u8> = hex_iter.map(|i|
                
                input.get(i..i+2)
                .and_then(|sub| 
                    u8::from_str_radix(sub, 16).ok()
                ).unwrap()
            ).collect();
            return Some(thing)
        }   
    }

    pub fn xor(&self, other: ByteString) -> Option<Vec<u8>> {
        println!("reffing bytes: {:?}", self.bytes.as_ref()?.len() != other.bytes.as_ref()?.len());
        
        if self.bytes.as_ref() == None || other.bytes.as_ref() == None {
            return None;
        }
        if self.bytes.as_ref()?.len() != other.bytes.as_ref()?.len() {
            return None;
        } else {
            let bytes_1 = self.bytes.as_ref().unwrap();
            let bytes_2 = other.bytes.as_ref().unwrap();

            let byte_iter = (0..bytes_1.len());

            let output: Vec<u8> = byte_iter.map(|index|
                bytes_1[index] ^ bytes_2[index]
            ).collect();

            Some(output)
        }
    }

    fn bytes_to_b64(input:&Vec<u8>) -> Option<Vec<u8>> {
        if input.len() % 3 != 0 {
            return None
        }
        let mut output:Vec<u8> = Vec::new();
        // iterate over vec of bytes, 3 at a time
        let byte_iter = (0..input.len()).step_by(3);
        for byte_i in byte_iter {
            let hex_bytes = input.get(byte_i..byte_i+3).unwrap();
            let byte_one:u8 = hex_bytes[0] >> 2u8;
            let byte_two:u8 = ((hex_bytes[0] & 0b00000011) << 4u8) + (hex_bytes[1] >> 4u8);
            let byte_three:u8 = ((hex_bytes[1] & 0b00001111) << 2u8) + ((hex_bytes[2] & 0b11000000) >> 6u8);
            let byte_four:u8 = (hex_bytes[2] & 0b00111111);
    
            let mut converted_bytes:Vec<u8> = vec![byte_one, byte_two, byte_three, byte_four];
            output.append(&mut converted_bytes);
        }
        return Some(output);
    }
    
    fn b64_to_string(input:&Vec<u8>) -> String {
        let dictionary: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
        let mut output_str: String = String::new();
        for byte in input {
            output_str.push(dictionary[*byte as usize])
        }
        return output_str
    }
    
}


