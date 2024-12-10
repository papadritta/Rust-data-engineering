use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

fn main() {
    let key = GenericArray::from_slice(b"an example key!!");
    let cipher = Aes128::new(&key);

    let plaintext = b"plaintext block!";
    let mut block = *GenericArray::from_slice(plaintext);

    cipher.encrypt_block(&mut block);
    println!("Encrypted: {:?}", block);

    cipher.decrypt_block(&mut block);
    println!("Decrypted: {:?}", block);
}
