mod constants;
mod decrypt;
mod encrypt;
mod key;
mod utils;

fn main() {
    //  Get type
    let choice = utils::get_input("Select type of operation:\n1 -> Encrypt\n2 -> Decrypt");

    if choice == "1".to_string() {
        let result = encrypt::encrypt();
        println!("After encryption: \n{:?}", result);
    } else if choice == "2".to_string() {
        let result = decrypt::decrypt();
        println!("After decryption: \n{:?}", result);
    } else {
        println!("Unknown operation. Shutting down")
    }
}
