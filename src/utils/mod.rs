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
        .step_by(s.len())
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

/**
 * Converts a byte vector to a string
 */
fn from_bytes_to_string(byte_vector: Vec<u8>) -> String {
    let word = format!("{:?}", byte_vector);
    let mut temp: Vec<&str> = word.split("").collect();

    temp.pop();
    temp.pop();
    temp.remove(0);
    temp.remove(0);

    temp.join("")
}

/**
 * Hexadecimal multiplication in regards to mix columns rules
 */
fn hex_multiplication(s1: String, s2: String) -> String {
    let l2 = decode_hex(s2.as_str()).unwrap();

    let l2_str = from_bytes_to_string(l2);
    let l2_num = l2_str.parse::<i64>().unwrap();

    if s1 == "03".to_string() {
        let mult_res = 2 * l2_num;
        let str_res = format!("{:x}", mult_res);
        if str_res.len() > 2 {
            let new_res = handle_overflow(str_res);
            return xor_two_hex_strings(new_res, s2);
        }
        return xor_two_hex_strings(str_res, s2);
    }

    let l1 = decode_hex(s1.as_str()).unwrap();
    let l1_str = from_bytes_to_string(l1);
    let l1_num = l1_str.parse::<i64>().unwrap();

    let res = l1_num * l2_num;
    let res_str = format!("{:02x}", res);
    if res_str.len() > 2 {
        return handle_overflow(res_str);
    }
    res_str
}

/**
 * Handles overflow after hex multiplication
 * More info: https://crypto.stackexchange.com/questions/57687/aes-encryption-algorithm-mix-columns
 */
fn handle_overflow(x: String) -> String {
    let handler = i64::from_str_radix("1b", 16).unwrap();
    let x = i64::from_str_radix(x.as_str(), 16).unwrap();

    let res = x ^ handler;
    let mut res_string = format!("{:02x}", res);
    res_string.remove(0);
    res_string
}

/**
 * Performs XOR to the given vector element
 */
fn vector_elements_xor(v: Vec<String>) -> String {
    let result_1 = xor_two_hex_strings(v[0].clone(), v[1].clone());
    let result_2 = xor_two_hex_strings(v[2].clone(), v[3].clone());
    xor_two_hex_strings(result_1, result_2)
}

/**
 * Performs dot product on the given hexadecimal vectors
 */
pub fn vector_dot_product(v1: Vec<&str>, v2: Vec<String>) -> String {
    let mut mult_vect = vec![String::new(); 4];

    for i in 0..4 {
        mult_vect[i] = hex_multiplication(v1[i].to_string().clone(), v2[i].clone());
    }

    vector_elements_xor(mult_vect)
}

/**
 * Calculates XOR of two hexadecimal strings
 */
fn xor_two_hex_strings(s1: String, s2: String) -> String {
    let s1_ints = i64::from_str_radix(s1.as_str(), 16).unwrap();
    let s2_ints = i64::from_str_radix(s2.as_str(), 16).unwrap();

    let res_int = s1_ints ^ s2_ints;
    format!("{:02x}", res_int)
}
