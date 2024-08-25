use std::collections::HashMap;
use crate::encoder;



pub fn main() {
    // Assume block size hase been determined from repeated calls
   
    // get encrypted version of a given block (must be 16 bytes long)
    let x = get_block("admin\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b".to_string());
    
    // get encrypted version of a profile where a new block starts exactly when role is defined
    let mut encrypted = Profile::encrypted_profile_cookie(&mut "xxxxxxxxxxxxx".to_string());
  
    // replace the last block of the real encrypted profile 
    // with our fabricated block.

                        // not crazy about this approach but splice() was
                        // behaving strangly and I am so tired
    for _ in 0..16 {
        encrypted.pop();
    }
    for i in 0..16 {
        encrypted.push(x[i]);
    }

    println!("here it is:{:?}", Profile::from_encrypted_cookie(encrypted));
}

fn get_block(mut plain_text: String) -> Vec<u8> {
    let first_block = "xxxxxxxxxx".to_string();
    if plain_text.len() > 16 {
        plain_text.truncate(16);
    }
    println!("plain text:{:?}", plain_text);
    let mut first_two_blocks = first_block + plain_text.as_str();
    println!("email is:{:?}", first_two_blocks);
    Profile::encrypted_profile_cookie(&mut first_two_blocks).get(16..32).unwrap().to_vec()
}

fn parse_cookie(cookie:&str) -> HashMap<&str, &str> {
    let mut output: HashMap<&str, &str> = HashMap::new();
    for pair in cookie.split("&") {
        let keyvalue: Vec<&str> =  pair.split("=").collect();
        output.insert(keyvalue[0], keyvalue[1]);
    }
    output
}

#[derive(Debug)]
struct Profile {
    email: String,
    uid: usize,
    role: String,
}

impl Profile {
    const KEY: [u8;16] = [25, 130, 156, 43, 47, 237, 129, 14, 162, 77, 206, 90, 44, 126, 247, 242];

    fn from_email(email:&mut str) -> Profile {
        let clean_email= email.replace("=", "").replace("&", "");
        Profile {
            email: clean_email,
            uid: (10 as usize),
            role: String::from("user"),
        }
    }

    fn encrypted_profile_cookie(email:&mut str) -> Vec<u8> {
        let profile = Profile::from_email(email);
        println!("unencrypted string:{:?}", profile.to_cookie());
        println!("1st 17:{:?}", profile.to_cookie().get(0..33));
        let mut cookie_bytes = profile.to_cookie().as_bytes().to_vec();
        encoder::pkcs7(&mut cookie_bytes, 16);
        println!("padded unencrypted:{:?}", &cookie_bytes);
        encoder::ebc_encrypt(&cookie_bytes, &Self::KEY)
    }

    fn from_encrypted_cookie(encrypted_cookie: Vec<u8>) -> Profile {
        let mut cookie_bytes = encoder::ebc_decrypt(&encrypted_cookie, &Self::KEY);
        println!("decrypted:{:?}", String::from_utf8(cookie_bytes.clone()));
        encoder::strip_pkcs7(&mut cookie_bytes);
        let cookie = String::from_utf8(cookie_bytes).unwrap();
        let cookie_hash = parse_cookie(&cookie);
        Profile{
            email: cookie_hash["email"].to_string(),
            uid: usize::from_str_radix(cookie_hash["uid"], 10).unwrap(),
            role: cookie_hash["role"].to_string(),
        }
    }

    fn to_cookie(&self) -> String {
        format!("email={}&uid={}&role={}", self.email, self.uid, self.role)
    }

    fn test() -> Profile {
        let mut cookie_bytes = "email=foo@bar.com&uid=10&role=user".as_bytes().to_vec();
        encoder::pkcs7(&mut cookie_bytes, 16);
        let encoded = encoder::ebc_encrypt(&cookie_bytes, &Self::KEY);
        Self::from_encrypted_cookie(encoded)
    }


}