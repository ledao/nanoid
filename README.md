# nanoid
nanoid is a rust version of [nanoid](https://github.com/aidarkhanov/nanoid), implemented by myself(not official).

# Usage
Open ```Cargo.toml``` file, and add dependencies.
```
[dependencies]
idnano = "0.8.1"
```

Generate a default nano id.
```rust
use idnano;

fn main() {
    let id: String = idnano::new();
    println!("id: {}", id); // id: g6-O7ul2xfd810SeN7Fjd for example.
}

```

Generate a customized nano id.
```rust
use idnano;

fn main() {
    let alphabet = "1234567890".as_bytes();
    let size = 10;
    let id: String = idnano::generate_string(alphabet, size);
    println!("id: {}", id); // id: 0548300922 for example.
}

```

# Thanks to

- [nanoid](https://github.com/aidarkhanov/nanoid)
