use crate::{black_box::{self, BlackBox}};
use crate::utils;

pub fn main(){
    let prefix = "comment1=cooking%20MCs;userdata=".as_bytes().to_vec();
    let suffix = ";comment2=%20like%20a%20pound%20of%20bacon".as_bytes().to_vec();
    let x = black_box::BlackBox::new_specific(prefix, suffix);

    // Prove that you can't inject admin=true without breaking crypto
    let mut encrypted = safe_encrypt(String::from(";admin=true;"), &x);
    let decrypted = x.cbc_decrypt(&mut encrypted);
    println!("Shouldn't work:  {:?}", String::from_utf8(decrypted));
    let admin = get_admin_status(&mut encrypted, &x);
    assert_eq!(admin, None);
    
    // find block size (16)
    let block_length = detect_block_length(&x);
    println!("Block length: {:?}", block_length);

    // use block size to build 2 block input. 
    // (Since prefix & suffix are known I'm ok with calling this a manual job)
    let pretransform = "XXXXXXXXXXXXXXXXXXXXX:admin<true";
    //                        0123456789abcdef0123456789abcdef

    // adjust indexes in block 1 to effect block 2 ';'s and '='s
    // This will scramble block 1... isn't that a problem?
    
    // comment1=cooking %20MCs;userdata= XXXXXXXXXXXXXXXX XXXXX:admin<true ;comment2=%20lik e%20a%20pound%20 of%20bacon\x06\x06\x06\x06\x06\x06
    // 0123456789abcdef 0123456789abcdef 0123456789abcdef 0123456789abcdef 0123456789abcdef 0123456789abcdef 0123456789a   b   c   d   e   f
    
    // Again, context suggests this doesn't need to be automated
    // so I'm just implementing block size manualy
    let mut encrypted = safe_encrypt(String::from(pretransform), &x);
    if encrypted[(16*2) + 5] % 2 == 0 {
        encrypted[(16*2) + 5] += 1;
    } else {
        encrypted[(16*2) + 5] -= 1;
    }
    
    if encrypted[(16*2) + 11] % 2 == 0 {
        encrypted[(16*2) + 11] += 1;
    } else {
        encrypted[(16*2) + 11] -= 1;
    }

    let admin = get_admin_status(&mut encrypted, &x);
    assert_eq!(admin, Some(true));

}

fn get_admin_status(encrypted: &mut Vec<u8>, black_box:&BlackBox) -> Option<bool>{
    let decrypted =  black_box.cbc_decrypt(encrypted);
    let plaintext = String::from_utf8_lossy(&decrypted);
    println!("{:?}", &plaintext);
    let admin: Vec<&str> = plaintext.matches(";admin=true;").collect();
    if admin.len() > 0 {
        Some(true)
    } else {
        None
    }   
}

fn block_print(block: &Vec<u8>) {
    let block_iter = (0..block.len()).step_by(16);
    for i in block_iter {
        println!("block index: {:?} block:{:?}", i, block.get(i..i+16));
    }
}

fn safe_encrypt(input: String, black_box: &BlackBox) -> Vec<u8> {
    let mut bytes = sanitize(input).as_bytes().to_vec();
    let encrypted = black_box.cbc_encrypt(&mut bytes);
    encrypted
}

fn sanitize(mut input: String) -> String {
    input = input.replace(";", "\";\"");
    input = input.replace("=", "\"=\"");

    input
}

pub fn detect_block_length(black_box: &BlackBox) -> Option<usize> {
    let mut current_streak = 1;
    let mut scnd_streak = 0;
    let mut thrd_streak = 0;
    let mut last_len = 0;
    for i in 0..100{
        let input = String::from_utf8(utils::test_input(i)).unwrap();
        let bytes = safe_encrypt(input, black_box);

        if bytes.len() == last_len {
            current_streak += 1;
        }
        if last_len != 0 && bytes.len() != last_len {
            thrd_streak = scnd_streak;
            scnd_streak = current_streak;
            current_streak = 0;
        }
        if current_streak > 0 && thrd_streak > 0 && current_streak == scnd_streak {
            println!("block is {:?} long", current_streak + 1);
            return Some(current_streak + 1);
        }

        last_len = bytes.len();
    }
    None
}