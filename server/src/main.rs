use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::Nonce;
use chacha20poly1305::{aead::generic_array::GenericArray, ChaCha20Poly1305, KeyInit};
use hex::decode;
use serde::Deserialize;

const CHACHA20POLY1305KEY: &str = "ab23fdecb2d314fc67ce38bff8e17780503849da862c1daab65b5ae3e66e19a7";
const CHACHA20POLY1305NONCE: &str = "6b79f6e6a9abbb1fa7b1ada7";

#[derive(Deserialize)]
struct EncryptedPayload {
    payload: String,
}

async fn encrypted(payload: web::Json<EncryptedPayload>) -> impl Responder {
    let encrypted_message_hex = &payload.payload;
    let encrypted_message = decode(encrypted_message_hex).unwrap();

    println!("Encrypted message: {:?}", encrypted_message);

    let hex_key_decoded = decode(CHACHA20POLY1305KEY).unwrap();
    let hex_nonce_decoded = decode(CHACHA20POLY1305NONCE).unwrap();
    let key = GenericArray::from_slice(&hex_key_decoded);
    let nonce = Nonce::from_slice(&hex_nonce_decoded);

    let cipher = ChaCha20Poly1305::new(&key);
    let decrypted_message = cipher.decrypt(&nonce, &*encrypted_message).unwrap();
    let decrypted_message_str = String::from_utf8(decrypted_message).unwrap();
    println!("Decrypted message: {}", decrypted_message_str);

    HttpResponse::Ok().body(decrypted_message_str)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/encrypted", web::post().to(encrypted)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
