use chacha20poly1305::{aead::generic_array, aead::Aead, ChaCha20Poly1305, KeyInit};
use generic_array::GenericArray;
use hex::{decode, encode};
use minreq::post;
use serde_json::json;

//change me!
const CHACHA20POLY1305KEY: &str = "ab23fdecb2d314fc67ce38bff8e17780503849da862c1daab65b5ae3e66e19a7";
//change me!
const CHACHA20POLY1305NONCE: &str = "6b79f6e6a9abbb1fa7b1ada7";

fn main() {
    //change me!
    let message = "Hello, World!";

    let hex_key_decoded = decode(CHACHA20POLY1305KEY).unwrap();
    let hex_nonce_decoded = decode(CHACHA20POLY1305NONCE).unwrap();

    let key = GenericArray::from_slice(&hex_key_decoded);
    let nonce = GenericArray::from_slice(&hex_nonce_decoded);

    let cipher = ChaCha20Poly1305::new(&key);
    let encrypted_message = cipher.encrypt(&nonce, message.as_ref()).unwrap();

    let encrypted_message_hex = encode(&encrypted_message);

    let payload = json!({
        "payload": encrypted_message_hex
    });

    let request = post("http://127.0.0.1:8080/encrypted")
        .with_header("Content-Type", "application/json")
        .with_body(payload.to_string());

    let response = request.send().unwrap();
    println!("{:?}", response);
}
