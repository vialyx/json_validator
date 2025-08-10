use json_validator::{load_json_file, parse_json};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <json-file>", args[0]);
        return;
    }

    let path = &args[1];
    match load_json_file(path) {
        Ok(raw) => match parse_json(&raw) {
            Ok(user) => {
                println!("✅ Valid JSON!");
                println!("{:#?}", user);
            }
            Err(err) => eprintln!("❌ Invalid JSON: {}", err),
        },
        Err(err) => eprintln!("❌ Failed to read file: {}", err),
    }
}