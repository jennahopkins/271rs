
diff <(cargo run --quiet -- src/main.rs) <(base64 -w 76 src/main.rs)
