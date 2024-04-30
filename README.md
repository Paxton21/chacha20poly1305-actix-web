# chacha20poly1305-actix-web
This is an example how to have encrypted communications to the actix-web server using chacha20poly1305 in Rust

# Encrypted Communications with actix-web using ChaCha20Poly1305

This project demonstrates secure communication between a client and a server using the ChaCha20Poly1305 encryption algorithm with the actix-web framework in Rust.

## Project Structure

The project consists of three main components:

1. **chacha20poly1305_gen**: Generates a random 256-bit key and a 96-bit nonce for the ChaCha20Poly1305 encryption algorithm.
2. **server**: An actix-web server that listens for incoming encrypted messages, decrypts them using the provided key and nonce, and responds with the decrypted message.
3. **client**: A client program that encrypts a message using the provided key and nonce, sends it to the server, and prints the server's response (the decrypted message).

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/Paxton21/chacha20poly1305-actix-web.git
```

2. Navigate to the `chacha20poly1305_gen` directory and run the program to generate the key and nonce:

```bash
cd chacha20poly1305_gen
cargo run
```

This will print out the key and nonce values in hexadecimal format, which you'll need in the next steps.

3. Open the `server/main.rs` file and replace the placeholder values for `CHACHA20POLY1305KEY` and `CHACHA20POLY1305NONCE` with the values generated in the previous step.

4. Navigate to the `server` directory and start the server:

```bash
cd server
cargo run
```

5. Open a new terminal window/tab and navigate to the `client` directory. Edit the `main.rs` file and replace the placeholder values for `CHACHA20POLY1305KEY`, `CHACHA20POLY1305NONCE`, and `message` with the appropriate values (use the same key and nonce from step 3, and set the desired message).

6. Run the client:

```bash
cd client
cargo run
```

The client will encrypt the message using the ChaCha20Poly1305 algorithm with the provided key and nonce, send the encrypted message to the server in hexadecimal format, and print the server's response (the decrypted message).

## Dependencies

This project uses the following Rust crates:

- **actix-web**: A powerful, pragmatic, and extremely high-performance web framework for Rust.
- **chacha20poly1305**: An implementation of the ChaCha20Poly1305 encryption algorithm.
- **rand**: A Rust library for random number generation.
- **hex**: Encoding and decoding hex strings.
- **serde**: A framework for serializing and deserializing Rust data structures efficiently and generically.
- **minreq**: A minimal HTTP client for Rust.

## License

This project is licensed under the [AGPLv3 License](LICENSE).
