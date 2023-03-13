# Websocket client/server demo


A portable websocket implementation in pure Rust.

## Example

```rust
cd ./Rust/websocket_server
cargo run

cd ./Rust/websocket_client
cargo run
```

## Optional ( Generate RSA KeyPairs)

```rust
cd ./Rust/RsaGenerator
cargo run

cp ./clientPrivate.key ../websocket_client/
cp ./clientPublic.key ../websocket_client/
cp ./serverPrivate.key ../websocket_server/
cp ./serverPublic.key ../websocket_server/
```

## Status

Currently at Phase 1 (v) 🚧

There will be next phases that refactor the project into a new library

1. 🚧  Set up environment ( If you haven't created key pairs)
    - [x] Generate key pairs for client and server ✅
    - [x] Copy client key pair to folder websocket_client ✅
    - [x] Copy server key pair to folder websocket_server ✅
    - [x] Run client and server cargo
2. 🚀 Flow
    - [x] Websocket Server will start and listen on 9001 port ✅
    - [x] Websocket Client will connect to the server via 9001 port ✅
3. 🔐 Make it secure
    - [x] Client will start exchange message to server
    - [x] Client gives its public key to server
    - [x] Server receives the client public key, and send back the server's public key
    - [x] Client and server start using RSA Encryption to encrypt data via this session
    - [x] Data encrypted by public key can only decrypt by its secret key
4. 🚧 Missing parts
    - [ ] The source code should be separated into a new library and client/server will import it as a dependency
    - [ ] As I am a newbie in Rust, I just spent 10 hours to study and implement this language,
   so the source code might not be well organized such as ( divided repeated code into smaller functions, etc. )


[//]: # (badges)

