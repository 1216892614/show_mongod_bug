# Replace the dependency from path to the 0.3.1 release to reproduce the error

```toml
[package]
name = "rust_playground"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
futures = "0.3.24"
mongod = { version = "0.3.1", features = ["derive"] } #<<<
thiserror = "1.0.37"
tokio = { version = "1.21.2", features = ["full"] }
```