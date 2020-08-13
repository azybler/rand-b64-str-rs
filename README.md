# Random_b64_str

This rust library provides a function to generate a random base-64 string from a random unsigned 128-bit integer. Useful for cases where you need a unique identifiers for Database record, or session tokens etc.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
random-b64-str = { git = "https://github.com/azybler/random-b64-str" }
```

## Example

```rust
use random_b64_str;

let random_b64_str = random_b64_str::get_u64();
println!("random_b64_str: {}", random_b64_str);

let random_b64_str = random_b64_str::get_u128();
println!("random_b64_str: {}", random_b64_str);
```
