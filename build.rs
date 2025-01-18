use serde_json::Value;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

// If you change these, make sure to change `src/lib.rs`
// Prost output file is `_.rs`
const OUTPUT_DIR: &str = "out/";
const CMD_ID_OUTPUT_FILE: &str = "cmd.rs";

// Source files
const PROTO_FILE: &str = "3.proto";
const CMD_ID_JSON: &str = "3.json";

fn main() {
    if !Path::new(OUTPUT_DIR).exists() {
        create_dir_all(OUTPUT_DIR).expect("Failed to create output directory");
    }

    if Path::new(PROTO_FILE).exists() {
        println!("cargo::rerun-if-changed={}", PROTO_FILE);

        prost_build::Config::new()
            .out_dir(OUTPUT_DIR)
            .compile_protos(&[PROTO_FILE], &["."])
            .expect("Failed to compile protobuf");
    } else {
        panic!("`{}` does not exist", PROTO_FILE);
    }

    if Path::new(CMD_ID_JSON).exists() {
        println!("cargo::rerun-if-changed={}", CMD_ID_JSON);

        let json_content = read_to_string(CMD_ID_JSON).expect("Failed to read JSON file");
        let parsed_json: Value =
            serde_json::from_str(&json_content).expect("Failed to parse JSON file");

        let constants = parsed_json
            .as_object()
            .expect("JSON file does not contain an object")
            .iter()
            .fold(String::new(), |mut acc, (key, value)| {
                let value = value
                    .as_u64()
                    .expect("Invalid value type, expected a number.")
                    as u16;
                let const_name = key
                    .as_str()
                    .chars()
                    .fold(String::new(), |mut acc, c| {
                        if c.is_uppercase() && !acc.is_empty() {
                            acc.push('_');
                        }
                        acc.push(c);
                        acc
                    })
                    .to_uppercase();
                acc.push_str(&format!("pub const {}: u16 = {};\n", const_name, value));
                acc
            });

        let output_path = format!("{}{}", OUTPUT_DIR, CMD_ID_OUTPUT_FILE);
        write(&output_path, constants).expect("Failed to write cmd id output file");
    } else {
        panic!("`{}` does not exist", CMD_ID_JSON);
    }
}
