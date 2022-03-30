mod vigenere;

use vigenere::*;


fn main() {
    let key = "WHYRUST";

    let plain_text = "TO EMPOWER EVERY ONE";
    let cipher_text = encrypt(plain_text, key);
    println!("Encrypting '{}' with key '{}' gives '{}'.", plain_text, key, cipher_text);


    let cipher_text = "PV CDJGPAY CMYJR KUC";
    let plain_text = decrypt(cipher_text, key);
    println!("Decrypting '{}' with key '{}' gives '{}'.", cipher_text, key, plain_text);
}
