# Rand_b64_str

This rust library provides a function to generate a random base-64 string from a random unsigned 128-bit integer. Useful for cases where you need a unique identifiers for Database record, or session tokens etc.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rand-b64-str = { git = "https://github.com/azybler/rand-b64-str-rs" }
```

## Example

```rust
use rand_b64_str;

let rand_b64_str = rand_b64_str::get_u64();
println!("rand_b64_str: {}", rand_b64_str);

let rand_b64_str = rand_b64_str::get_u128();
println!("rand_b64_str: {}", rand_b64_str);
```
