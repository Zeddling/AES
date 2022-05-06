mod constants;
mod decrypt;
mod encrypt;
mod key;
mod utils;

fn main() {
    let result = encrypt::encrypt();
    println!("After encryption: \n{:?}", result);
}
