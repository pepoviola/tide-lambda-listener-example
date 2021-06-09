use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    if target_env == "musl" {
        println!("cargo:warning= BUILDING FOR LAMBDA");
        let lambda_fn =
            env::var("CARGO_LAMBDA_FN").expect("Invalid env var CARGO_USE_LAMBDA_FN, must be set");
        let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(format!("src/functions/{}.rs", lambda_fn));
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("lambda.rs");
        fs::copy(input_path, dest_path).unwrap();
    } else {
        println!("cargo:warning= NO BUILDING FOR LAMBDA");
    }
}
