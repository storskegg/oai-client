use std::env;
use std::fs;

fn main() {
    let semver = env!("CARGO_PKG_VERSION"); // like "0.2.1"
    let out_dir = ".build_metadata"; // You can put this in OUT_DIR if you prefer

    // Ensure metadata directory exists
    fs::create_dir_all(out_dir).unwrap();

    // Path for the build count file
    let build_count_path = format!("{}/build-count.txt", out_dir);

    // Read the old file (if any)
    let mut previous_version = String::new();
    let mut previous_count: u32 = 0;

    if let Ok(contents) = fs::read_to_string(&build_count_path) {
        let mut lines = contents.lines();
        previous_version = lines.next().unwrap_or("").to_string();
        previous_count = lines.next().and_then(|line| line.parse().ok()).unwrap_or(0);
    }

    // If semver changed, reset to 0; else, increment
    let new_count = if semver != previous_version {
        1
    } else {
        previous_count + 1
    };

    // Save updated info
    let contents = format!("{semver}\n{new_count}");
    fs::write(&build_count_path, contents).unwrap();

    // Export via cargo environment vars
    println!("cargo:rustc-env=BUILD_SEMVER={}", semver);
    println!("cargo:rustc-env=BUILD_COUNT={}", new_count);
    println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=path/to/source.txt"); // etc
}
