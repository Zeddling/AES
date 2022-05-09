use crate::utils::{divide_into_state, xor_matrices, Block};

pub fn encrypt() -> Block {
    println!("Welcome!");
    let plaintext = crate::utils::get_input("Enter the plain text");
    let mut passphrase = crate::utils::get_input("Enter the passphrase: (16 characters)");

    while !crate::utils::input_is_valid(passphrase.as_str()) {
        println!("Invalid input!");
        passphrase = crate::utils::get_input("Enter the passphrase: (16 characters)");
    }

    //  Pad plaintext
    // let normalized_plaintext = crate::utils::normalize(&plaintext);

    //  Create state
    let mut state: Block = divide_into_state(plaintext);
    let key: Block = divide_into_state(passphrase);

    println!("State \n{:?}", state);
    println!("Key \n{:?}", key);

    let mut round_keys = crate::key::RoundKeys { keys: vec![key] };
    round_keys.generate();

    //  Step 0
    state = xor_matrices(round_keys.keys[0].clone(), state);
    println!("{:?}", state);

    //  Step 1 - 9
    for i in 1..10 {
        state = substitute_bytes(state);
        state = shift_rows(state);
        state = mix_columns(state);
        state = xor_matrices(round_keys.keys[i].clone(), state);
    }
    //  Step 10
    state = substitute_bytes(state);
    state = shift_rows(state);
    xor_matrices(round_keys.keys[10].clone(), state)
}

/**
 * Substitutes the hex values using SBOX matrix
 */
fn substitute_bytes(block: Block) -> Block {
    let mut result_block: Block = vec![vec![String::new(); 4]; 4];

    for j in 0..4 {
        let mut sub_vect = vec![String::new(); 4];

        let mut word_index: usize = 0;
        for word in &block[j] {
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
        result_block[j] = sub_vect;
    }
    result_block
}

/**
 * Cyclically shifts the position of entries in each row of the matrix
 */
fn shift_rows(block: Block) -> Block {
    //  Create new block to have the state block row wise rather than
    //  column wise
    let mut temp_block_rows = vec![vec![String::new(); 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            temp_block_rows[j][i] = block[i][j].clone();
        }
    }

    //  Rotate left
    for i in 0..4 {
        temp_block_rows[i].rotate_left(i);
    }

    //  Create new block and fill words column wise
    let mut result_block = vec![vec![String::new(); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result_block[i][j] = temp_block_rows[j][i].clone();
        }
    }

    result_block
}

/**
 * Multiplies fixed matrix against the current state matrix
 */
fn mix_columns(block: Block) -> Block {
    let mut result_block: Block = vec![vec![String::new(); 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            result_block[i][j] = crate::utils::vector_dot_product(
                crate::constants::MIX_COLUMNS_MATRIX[j].to_vec(),
                block[i].clone(),
                true,
            )
        }
    }

    result_block
}
