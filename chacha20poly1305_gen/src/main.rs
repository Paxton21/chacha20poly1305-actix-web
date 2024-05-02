use hex::encode;
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::File;
use std::io::Write;

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


	//Create the environment variables
	let key_env_var = "KEY_VALUE";
	let nonce_env_var = "NONCE_VALUE";

	//format the output env value file with var and values
	let env_value = format!("{}={}\n{}={}", key_env_var, key_hex, nonce_env_var, nonce_hex);

	// Set the preferred file name and then write to file
	let file = ".env";
	let mut env_file = File::create(file).expect("Unable to create env file. Are permissions setup properly?");
	env_file.write_all(
		env_value.as_bytes()
	).expect("Failed to write to env file");
    
    println!(".env file updated with key and Nonce");
}
