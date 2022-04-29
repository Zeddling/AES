fn divide_into_state(plaintext: String) -> Vec<Vec<String>> {
    let text_vect = crate::utils::to_vec(plaintext);
    let text_chunks = text_vect.chunks(4);
    let mut state = vec![vec![String::new()]];

    for chunk in text_chunks {
        state.push(chunk.to_owned());
    }

    //  Remove initial block
    state.remove(0);

    return state;
}

pub fn encrypt() {
    println!("Welcome!");
    let data: (String, String) = crate::utils::get_input();
    let plaintext = data.0;

    //  Create state
    let mut state = divide_into_state(plaintext);
    let key = divide_into_state(data.1);

    println!("State \n{:?}", state);
    println!("Key \n{:?}", key);

    let mut round_keys = crate::key::RoundKeys { keys: vec![key] };
    round_keys.generate();
}
