use vigenere::Vigenere;

const KEY: &str = "WHYRUST";

fn main() {
    let cypher = Vigenere::new(KEY);

    let plain_text = "TO EMPOWER EVERYONE";
    let cipher_text = cypher.encrypt(plain_text);
    println!("Encrypting '{}' with key '{}' gives '{}'.", plain_text, KEY, cipher_text);

    let cipher_text = "PV CDJGPAY CMYJRKUC";
    let plain_text = cypher.decrypt(cipher_text);
    println!("Decrypting '{}' with key '{}' gives '{}'.", cipher_text, KEY, plain_text);
}
