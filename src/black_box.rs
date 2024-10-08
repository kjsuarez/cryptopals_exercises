use crate::encoder;
use rand::Rng;
use base64::prelude::*;
use crate::pkcs7::*;


#[derive(Debug, Clone)]
pub struct BlackBox{
    key: [u8;16],
    prefix: Vec<u8>,
    suffix: Vec<u8>,
}

impl BlackBox {
    const SECRET: &'static str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    
    fn random_bytes() -> Vec<u8>{
        let mut out: Vec<u8> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..100) {
            out.push(rng.gen::<u8>());
        }
        out
    }

    pub fn new_specific(prefix:Vec<u8>, suffix: Vec<u8>) -> BlackBox{
        BlackBox{
            key: encoder::random_aes_key(),
            prefix: prefix,
            suffix: suffix
        }
    }

    pub fn new_random_prefix() -> BlackBox{
        BlackBox{
            key: encoder::random_aes_key(),
            prefix: Self::random_bytes(),
            suffix: "".as_bytes().to_vec()
        }
    }

    pub fn new() -> BlackBox{
        BlackBox{
            key: encoder::random_aes_key(),
            prefix: "".as_bytes().to_vec(),
            suffix: "".as_bytes().to_vec()
        }
    }

    pub fn new_stable() -> BlackBox{
        BlackBox{
            key: *b"YELLOW SUBMARINE",
            //    0123456789abcdef
            prefix: "".as_bytes().to_vec(),
            suffix: "".as_bytes().to_vec()
        }
    }

    pub fn new_with_short_prefix() -> BlackBox{
        BlackBox{
            key: encoder::random_aes_key(),
            prefix: "0123".as_bytes().to_vec(),
            suffix: "".as_bytes().to_vec()
        }
    }

    pub fn new_with_big_prefix() -> BlackBox{
        BlackBox{
            key: encoder::random_aes_key(),
            prefix: "0123456789abcdef0".as_bytes().to_vec(),
            suffix: "".as_bytes().to_vec()
        }
    }

    pub fn encrypt_with_prefix(&self, input: &mut Vec<u8>) -> Vec<u8>{
        let mut secret_bytes = BASE64_STANDARD.decode(Self::SECRET.as_bytes()).unwrap();
        let mut plain: Vec<u8> = Vec::new();
        let mut prefix = self.prefix.clone();
        // append secret bytes to end of input
        plain.append(&mut prefix);
        plain.append(input);
        plain.append(&mut secret_bytes);
        // ecb encrypt result with related key
        pkcs7(&mut plain, 16);
        let encrypted = encoder::ebc_encrypt(&plain, &self.key);
        // return result
        encrypted
    }

    pub fn cbc_encrypt(&self, input: &mut Vec<u8>) -> Vec<u8> {
        let mut plain: Vec<u8> = Vec::new();
        let mut prefix = self.prefix.clone();
        let mut suffix = self.suffix.clone();
        // append secret bytes to end of input
        plain.append(&mut prefix);
        plain.append(input);
        plain.append(&mut suffix);
        pkcs7(&mut plain, 16);
        encoder::cbc_ecrypt(&plain, &self.key)
    }

    pub fn cbc_encrypt_with_iv(&self,iv: &[u8], input: &mut Vec<u8>) -> Vec<u8> {
        let mut plain: Vec<u8> = Vec::new();
        let mut prefix = self.prefix.clone();
        let mut suffix = self.suffix.clone();
        // append secret bytes to end of input
        plain.append(&mut prefix);
        plain.append(input);
        plain.append(&mut suffix);
        pkcs7(&mut plain, 16);
        encoder::cbc_ecrypt_with_iv(iv,&plain, &self.key)
    }

    pub fn cbc_decrypt(&self, input: &mut Vec<u8>) -> Vec<u8> {
        let mut input = encoder::cbc_decrypt(input, &self.key);
        strip_pkcs7(&mut input);
        input
    }

    pub fn cbc_decrypt_keep_padding(&self, input: &mut Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
        let input = encoder::cbc_decrypt_with_iv(iv, input, &self.key);
        // strip_pkcs7(&mut input);
        input
    }
}



