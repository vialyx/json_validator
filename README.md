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
