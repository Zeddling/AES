use std::io;
use std::{fmt::Write, num::ParseIntError};

pub type Block = Vec<Vec<String>>;

/**
 * Returns 2 hexadecimal string arrays (plaintext, passphrase)
 *
 */
pub fn get_input(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    //hex::encode(message.trim().to_string()),
    input.trim().to_string()
}

pub fn normalize(text: &String) -> String {
    let text_len = text.len();

    //  Apply padding
    if text_len <= 8 {
        //  Will give us the required padding to make the
        //  string divisible by 16
        let padding = text_len % 16;
        let res = format!("{:width$}", text, width = text_len + padding);
        return hex::encode(res);
    } else {
        //  find next largest number then get modulus
        let padding = ((text_len / 16 + 1) * 16) % text_len;
        let res = format!("{:width$}", text, width = text_len + padding);
        return hex::encode(res);
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
            let new_res = handle_overflow_string(str_res);
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
        return handle_overflow_string(res_str);
    }
    res_str
}

/**
 * TODO:
 * https://crypto.stackexchange.com/questions/2569/how-does-one-implement-the-inverse-of-aes-mixcolumns
 */
fn hex_multiplication_decrypt(s1: &str, s2: String) -> String {
    //  Decode second string first
    let s2_bytes = decode_hex(s2.as_str()).unwrap();

    let s2_str = from_bytes_to_string(s2_bytes);
    let s2_num = s2_str.parse::<i64>().unwrap();

    if s1 == crate::constants::NINE {
        //  x*9=(((x*2)*2)*2)+x
        let mut answer = s2_num * 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);

        format!("{:02x}", answer ^ s2_num)
    } else if s1 == crate::constants::ELEVEN {
        //  x*11=((((x*2)*2)+x)*2)+x
        let mut answer = s2_num * 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer ^= s2_num;
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);

        return format!("{:02x}", answer ^ s2_num);
    } else if s1 == crate::constants::THIRTEEN {
        //  x*13=((((x*2)+x)*2)*2)+x
        let mut answer = s2_num * 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer ^= s2_num;
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        return format!("{:02x}", answer ^ s2_num);
    } else {
        //  x*14=((((x*2)+x)*2)+x)*2
        let mut answer = s2_num * 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer ^= s2_num;
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        answer ^= s2_num;
        answer *= 2;
        answer = check_overflow_hex_multiplication_decrypt(answer);
        return format!("{:02x}", answer);
    }
}

/**
 * Handles overflow after hex multiplication
 * More info: https://crypto.stackexchange.com/questions/57687/aes-encryption-algorithm-mix-columns
 */
fn handle_overflow_string(x: String) -> String {
    let handler = i64::from_str_radix("1b", 16).unwrap();
    let x = i64::from_str_radix(x.as_str(), 16).unwrap();

    let res = x ^ handler;
    let mut res_string = format!("{:02x}", res);
    res_string.remove(0);
    res_string
}

/**
 * Handles overflow after hex multiplication
 * More info: https://crypto.stackexchange.com/questions/57687/aes-encryption-algorithm-mix-columns
 */
fn handle_overflow_int(x: i64) -> i64 {
    let handler = i64::from_str_radix("1b", 16).unwrap();

    let res = x ^ handler;
    let mut res_string = format!("{:02x}", res);
    res_string.remove(0);
    i64::from_str_radix(res_string.as_str(), 16).unwrap()
}

fn check_overflow_hex_multiplication_decrypt(x: i64) -> i64 {
    if format!("{:02x}", x).len() > 2 {
        return handle_overflow_int(x);
    }
    x
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
pub fn vector_dot_product(v1: Vec<&str>, v2: Vec<String>, is_encryption: bool) -> String {
    let mut mult_vect = vec![String::new(); 4];

    for i in 0..4 {
        if is_encryption {
            mult_vect[i] = hex_multiplication(v1[i].to_string().clone(), v2[i].clone());
        } else {
            mult_vect[i] = hex_multiplication_decrypt(v1[i].clone(), v2[i].clone());
        }
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

pub fn input_is_valid(text: &str) -> bool {
    if text.len() != 16 {
        return false;
    }
    true
}

pub fn xor_matrices(matrice1: Block, matrice2: Block) -> Block {
    let mut result_block: Block = vec![vec![String::new(); 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            let ij_matrice1 = decode_hex(matrice1[i][j].as_str()).unwrap();
            let ij_matrice2 = decode_hex(matrice2[i][j].as_str()).unwrap();

            let result_vector: Vec<u8> = ij_matrice1
                .iter()
                .zip(ij_matrice2.iter())
                .map(|(&x1, &x2)| x1 ^ x2)
                .collect();

            let result_string = encode_hex(result_vector.as_slice());
            result_block[i][j] = result_string;
        }
    }

    result_block
}

pub fn divide_into_state(text: String) -> Vec<Vec<String>> {
    let text_vect = to_vec(text);
    let text_chunks = text_vect.chunks(4);
    let mut state: Block = vec![vec![String::new()]];

    for chunk in text_chunks {
        state.push(chunk.to_owned());
    }

    //  Remove initial block
    state.remove(0);

    return state;
}
