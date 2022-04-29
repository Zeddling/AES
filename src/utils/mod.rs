use std::io;
use std::{fmt::Write, num::ParseIntError};

/**
 * Returns 2 hexadecimal string arrays (plaintext, passphrase)
 *
 */
pub fn get_input() -> (String, String) {
    println!("Enter a message: ");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Error reading input");

    println!("Enter a passphrase: ");
    let mut passphrase = String::new();
    io::stdin()
        .read_line(&mut passphrase)
        .expect("Error reading input");

    (
        //hex::encode(message.trim().to_string()),
        message.trim().to_string(),
        //hex::encode(passphrase.trim().to_string()),
        passphrase.trim().to_string(),
    )
}

pub fn normalize(text: &String) -> String {
    let text_len = text.len();

    //  Apply padding
    if text_len <= 8 {
        //  Will give us the required padding to make the
        //  string divisible by 16
        let padding = text_len % 16;
        return format!("{:width$}", text, width = text_len + padding);
    } else {
        //  find next largest number then get modulus
        let padding = ((text_len / 16 + 1) * 16) % text_len;
        return format!("{:width$}", text, width = text_len + padding);
    }
}

pub fn to_vec(text: String) -> Vec<String> {
    let mut str_vec: Vec<String> = vec![String::new(); 2];
    let mut text_vec: Vec<&str> = text.split("").collect();

    //  Remove leading and trailing ""
    text_vec.remove(0);
    text_vec.remove(text_vec.len() - 1);

    for i in 0..text_vec.len() {
        let j = i + 1;

        //  means i is on an already visited index
        if j % 2 == 0 {
            continue;
        }

        str_vec.push(format!("{}{}", text_vec[i], text_vec[j]));
    }
    //  remove first trailing spaces
    str_vec.remove(0);
    str_vec.remove(0);

    str_vec
}

//  Hex decoder

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}
