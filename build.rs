use std::process::Command;

use chrono::Utc;

fn main() {
    // set build time
    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP={}",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    let dirty = Command::new("git").args(["status", "--porcelain"]).output();

    let dirty = match dirty {
        Ok(output) if output.status.success() && !output.stdout.is_empty() => "-dirty",
        _ => "",
    };

    // retrieve git hash
    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();

    match hash {
        Ok(output) if output.status.success() => {
            println!(
                "cargo:rustc-env=GIT_HASH={}{dirty}",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
        _ => println!("cargo:rustc-env=GIT_HASH="),
    }
}
