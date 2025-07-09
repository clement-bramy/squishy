use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let timestamp = format!("{}", timestamp);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
}
