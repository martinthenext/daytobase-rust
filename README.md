# Daytobase in Rust

Set env variables by creating `.env` file from `.env.template`, then run it like:

```sh
cargo build
export $(cat .env | xargs) && RUST_LOG=info && ./target/release/daytobase-rust
```
