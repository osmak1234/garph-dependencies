use serde_json::Value;
use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("cargo")
        .arg("metadata")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();

    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();

    let parsed: Value = read_json(&stdout);

    let dependencies: Value = parsed["packages"][0]["dependencies"].clone();

    let mut no_more_deps = false;

    let mut i = 0;

    while !no_more_deps {
        if let Some(obj) = dependencies.get(i) {
            println!("{}", obj["name"]);
            i += 1;
        } else {
            no_more_deps = true;
        }
    }
}

fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    parsed
}
