



use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("feature_cfg.rs");
    
    let enable_pass_feature = env::var("ENABLE_PASS_FEATURE").is_ok();

    let mut file = File::create(&dest_path).unwrap();
    
    if enable_pass_feature {
        file.write_all(b"pub const ENABLE_PASS_FEATURE: bool = true;\n").unwrap();
    } else {
        file.write_all(b"pub const ENABLE_PASS_FEATURE: bool = false;\n").unwrap();
    }
}


