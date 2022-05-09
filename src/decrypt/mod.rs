use crate::utils::{divide_into_state, xor_matrices, Block};

pub fn decrypt() -> Block {
    let ciphertext = crate::utils::get_input("Enter the cipher text");
    let mut passphrase = crate::utils::get_input("Enter the passphrase: (16 characters)");

    while !crate::utils::input_is_valid(passphrase.as_str()) {
        println!("Invalid input!");
        passphrase = crate::utils::get_input("Enter the passphrase: (16 characters)");
    }

    let mut state: Block = divide_into_state(ciphertext);
    let key: Block = divide_into_state(passphrase);

    let mut round_keys = crate::key::RoundKeys { keys: vec![key] };
    round_keys.generate();

    let mut i: usize = 10;

    //  Step 0
    println!("round[ {}] istart: {:?}", 0, round_keys.keys[i]);
    state = xor_matrices(round_keys.keys[i].clone(), state);
    i -= 1;

    //  Step 1 - 9
    while i > 0 {
        println!("round[ {}] istart: {:?}", 10 - i, state);

        state = inv_substitute_bytes(state);
        println!("round[ {}] is_box: {:?}", 10 - i, state);

        state = inv_shift_rows(state);
        println!("round[ {}] is_row: {:?}", 10 - i, state);

        state = inv_mix_columns(state);
        println!("round[ {}] im_col: {:?}", 10 - i, state);

        let keyn = inv_mix_columns(round_keys.keys[i].clone());
        println!("round[ {}] ik_sch: {:?}", 10 - i, keyn);

        state = xor_matrices(keyn, state);
        println!("round[ {}] ik_add: {:?}", 10 - i, state);

        i -= 1;
    }
    //  Step 10
    state = inv_substitute_bytes(state);
    println!("round[ {}] is_box: {:?}", 10, state);

    state = inv_shift_rows(state);
    println!("round[ {}] is_row: {:?}", 10, state);

    println!("round[ {}] ik_sch: {:?}", 10, round_keys.keys[0].clone());
    xor_matrices(round_keys.keys[0].clone(), state)
}

fn inv_shift_rows(block: Block) -> Block {
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
        temp_block_rows[i].rotate_right(i);
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

fn inv_substitute_bytes(block: Block) -> Block {
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

            sub_vect[word_index] =
                crate::constants::INV_SBOX_MATRIX[indices[0]][indices[1]].to_string();

            word_index += 1;
        }
        result_block[j] = sub_vect;
    }
    result_block
}

fn inv_mix_columns(block: Block) -> Block {
    let mut result_block: Block = vec![vec![String::new(); 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            result_block[i][j] = crate::utils::vector_dot_product(
                crate::constants::INV_MIX_COLUMNS_MATRIX[j].to_vec(),
                block[i].clone(),
                false,
            )
        }
    }

    result_block
}
