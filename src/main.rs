extern crate crypto;
//extern crate clipboard;
extern crate base64;
extern crate x11_clipboard;

use std::env;
use x11_clipboard::Clipboard;

struct Filter {
    /// Struct used to filter password characters.
    lowercase:  bool,
    uppercase:  bool,
    digits:     bool,
    specials:   bool,
}

impl Filter {
    pub fn new(lowercase: bool, uppercase: bool, digits: bool, specials: bool)
            -> Filter {
        Filter {
            lowercase:  lowercase,
            uppercase:  uppercase,
            digits:     digits,
            specials:   specials,
        }
    }
}

fn main() {

    // argv
    let mut arguments: Vec<String> = env::args().collect();

    // scrypt parameters
    let salt:     String = arguments.pop().unwrap();
    let password: String = arguments.pop().unwrap();
    let salt_u8:     &[u8] = salt.as_bytes();
    let password_u8: &[u8] = password.as_bytes();
    let output: &mut [u8] = &mut [0;32];
    let params = &crypto::scrypt::ScryptParams::new(1, 8, 1); //TODO:choose parameters

    // key function
    crypto::scrypt::scrypt(password_u8, salt_u8, params, output);

    // [u8] to base64 to have a string
    let result: String = base64::encode(output);
    println!("{}", result);

    // clip the string
    // TODO: beautiful functions?
    let clipboard = Clipboard::new().unwrap();
    let val =
        clipboard.store(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            result
        )
        .unwrap();
    let val2 =
        clipboard.load_wait(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property
        )
        .unwrap();
}
