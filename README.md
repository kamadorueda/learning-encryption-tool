# Usage

```sh
alias encryption-tool="cargo run --release --"
encryption-tool encrypt --in Cargo.toml --out encrypted
encryption-tool show --in encrypted
encryption-tool decrypt --in encrypted --out decrypted
cat decrypted
diff Cargo.toml decrypted && echo no diff
```

# Tests

```sh
cargo test
```
