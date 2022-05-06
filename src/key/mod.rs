//  Key expansion
//  Process(n-1 steps): in our case 9 to have keys for 10 rounds
//      1. Take left shifted of last column of key n -> RotWord
//      2. Perform SubByte() on column (Sbox conversion)
//      3. Take j column of key state n where 0 <= j < 4
//      4. To get first column of key n+1:
//          First column of key n XOR SubByted column XOR RCON[n]
//      5. To get next columns of Key n+1
//          m(where 0<=m<4) column of key n+1 XOR K(where 0<K<4) column of key n

/**
 * Represents a key
 *
 * Properties:
 *      1. Keys can only have 4 rows and 4 columns
 *      2. Keys are in hexadecimal format
 */
pub type Key = Vec<Vec<String>>;

/**
 * A 3D array representing all the keys generated from the expansion
 */
pub struct RoundKeys {
    pub keys: Vec<Key>,
}

impl RoundKeys {
    pub fn generate(&mut self) {
        //  XOR operation
        for i in 0..10 {
            //  Perform  RotWord permutation
            let rot = self.rot_word(i);

            //  Perform SubByte
            let sub_byte_vector = self.sub_byte(rot);

            self.generate_key_columns(sub_byte_vector, i);
        }
    }

    /**
     * Performs rotation(left shift) on 3 word of current key
     */
    fn rot_word(&self, key_index: usize) -> Vec<String> {
        let mut third_word = self.keys[key_index][3].clone();

        //  left cyclical rotation
        third_word.rotate_left(1);

        third_word
    }

    fn sub_byte(&self, rot: Vec<String>) -> Vec<String> {
        let mut sub_vect = vec![String::new(); 4];
        //  Trial run -> Use first letter in vector

        let mut word_index: usize = 0;
        for word in rot {
            let mut r: Vec<&str> = word.split("").collect();
            r.pop();
            r.remove(0);

            // Populate indices and retrieve value from SBOX
            let mut index: usize = 0;

            //  Initialize indices for SBOX
            let mut indices: [usize; 2] = [0; 2];
            for i in r {
                //  If it's not a letter, we expect a number
                if crate::constants::HEX_LETTERS.contains(&i) {
                    let letter = crate::constants::HEX_LETTERS
                        .iter()
                        .position(|&j| j == i)
                        .unwrap();
                    indices[index] = letter + 10;
                } else {
                    indices[index] = i.parse::<usize>().unwrap();
                }
                index += 1;
            }

            sub_vect[word_index] = crate::constants::SBOX[indices[0]][indices[1]].to_string();

            word_index += 1;
        }

        sub_vect
    }

    fn generate_key_columns(&mut self, sub_byte: Vec<String>, process_num: usize) {
        //  Add round constant
        let rcon = crate::utils::decode_hex(crate::constants::RCON[process_num]).unwrap();
        let first_in_sub_byte = crate::utils::decode_hex(sub_byte[0].clone().as_str()).unwrap();

        //  Do xor operation
        let res: Vec<u8> = rcon
            .iter()
            .zip(first_in_sub_byte.iter())
            .map(|(&x1, &x2)| x1 ^ x2)
            .collect();

        //  Update vector
        let result_string = crate::utils::encode_hex(res.as_slice());
        let mut rcon_result = sub_byte;
        rcon_result[0] = result_string;

        //  Initialize new key
        let mut new_key = vec![vec![String::new()]];

        //  Calculate first column of new key
        new_key.push(self.xor_words(rcon_result, self.keys[process_num][0].clone()));
        //  Remove first value of initialization
        new_key.remove(0);

        for i in 1..4 {
            new_key.push(self.xor_words(new_key[i - 1].clone(), self.keys[process_num][i].clone()));
        }
        self.keys.push(new_key);
    }

    fn xor_words(&self, word1: Vec<String>, word2: Vec<String>) -> Vec<String> {
        let mut result_vector = vec![String::new(); 4];
        for i in 0..4 {
            let letter_word1_bytes = crate::utils::decode_hex(word1[i].as_str()).unwrap();
            let letter_word2_bytes = crate::utils::decode_hex(word2[i].as_str()).unwrap();

            let result: Vec<u8> = letter_word1_bytes
                .iter()
                .zip(letter_word2_bytes.iter())
                .map(|(&x1, &x2)| x1 ^ x2)
                .collect();

            result_vector[i] = crate::utils::encode_hex(result.as_slice());
        }

        result_vector
    }
}
