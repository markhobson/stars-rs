# Rust Demo

Wasm graphics demo using Rust.

Following [Making really tiny WebAssembly graphics demos](https://cliffle.com/blog/bare-metal-wasm).

## Running the demo

1. Build the project:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

1. Serve the project:

   ```bash
   npm install
   npm start
   ```

The demo will open http://127.0.0.1:8080/.

## Running the tests

Run the tests:

```bash
cargo test
```
