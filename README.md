# 🦀 Rust JSON Parser & Validator

A simple CLI tool written in Rust to **parse, sanitize, and validate JSON files** using `serde` and `serde_json`.

## 📌 Features
- **Read JSON** from a file.
- **Sanitize input** by removing unsafe control characters.
- **Validate structure** against a predefined Rust data model.
- **Pretty-print** valid JSON for better readability.

## 🛠 Tech Stack
- [Rust](https://www.rust-lang.org/)
- [Serde](https://serde.rs/) — serialization/deserialization.
- [serde_json](https://docs.rs/serde_json) — JSON handling.

## 🚀 Getting Started

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone the Repo
```bash
git clone https://github.com/YOUR_USERNAME/json_parser_validator.git
cd json_parser_validator
```

### 3. Build & Run
```bash
cargo run -- path/to/input.json
```

### 📂 Project Structure
```bash
src/
├── lib.rs   # Core logic: sanitization, parsing, validation
└── main.rs  # CLI entry point
Cargo.toml   # Dependencies & metadata
```

### 🧪 Example
```json
{
    "name": "Alice",
    "age": 30
}
```
Run:
```bash
cargo run -- data.json
```
Output:
```bash
✅ JSON is valid!
{
  "name": "Alice",
  "age": 30
}
```
