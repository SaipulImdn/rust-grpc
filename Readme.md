# rust-grpc

A simple gRPC microservice built in Rust using Clean Architecture principles.

---

## 🛠️ Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Tonic](https://docs.rs/tonic/) – gRPC implementation in Rust
- [Prost](https://docs.rs/prost/) – Protocol Buffers support
- [Tokio](https://tokio.rs/) – Asynchronous runtime
- [Protobuf](https://developers.google.com/protocol-buffers)

---

## 📁 Project Structure

```
rust-grpc/
├── Cargo.toml
├── build.rs
├── proto/
│   └── greeter.proto
├── src/
│   ├── main.rs
│   ├── domain/
│   │   └── mod.rs
│   ├── usecase/
│   │   └── mod.rs
│   ├── infrastructure/
│   │   └── mod.rs
│   └── presentation/
│       └── mod.rs
```

- **proto/**: Protocol Buffer definitions.
- **domain/**: Core traits and business entities.
- **usecase/**: Application use cases and logic.
- **infrastructure/**: gRPC server implementation and external layers.
- **presentation/**: Server interface.
- **main.rs**: Application entry point.

---

## 🚀 Getting Started

### 1. Install `protoc` (Protocol Buffers Compiler)

**On Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y protobuf-compiler
```

**On macOS (Homebrew):**
```bash
brew install protobuf
```

**Verify installation:**
```bash
protoc --version
```

### 2. Build the project

```bash
cargo build
```

### 3. Run the gRPC server

```bash
cargo run
```

---

## 🧪 Example `greeter.proto`

```proto
syntax = "proto3";

package greeter;

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloReply);
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
}
```

---

## 📦 Example `Cargo.toml` Dependencies

```toml
[dependencies]
tonic = "0.10"
prost = "0.12"
tokio = { version = "1", features = ["full"] }

[build-dependencies]
tonic-build = "0.10"
```

---

## 🔨 Build Script (`build.rs`)

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/greeter.proto")?;
    Ok(())
}
```

---

## ✨ Credits

This project is created as a base template for building gRPC microservices in Rust using a clean and modular architecture.