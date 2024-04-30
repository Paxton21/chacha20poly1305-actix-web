use hex::encode;
use rand::rngs::OsRng;
use rand::RngCore;

fn main() {
    let mut rng = OsRng;
    let mut key = vec![0u8; 32]; 
    let mut nonce = vec![0u8; 12]; 

    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut nonce);

    let key_hex = encode(key);
    let nonce_hex = encode(nonce);

    println!("ChaCha20 Key (hex): {}", key_hex);
    println!("ChaCha20 Nonce (hex): {}", nonce_hex);
}