use std::env;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    cc::Build::new().file("some-c/gcd.c").compile("gcd");

    let bindings = bindgen::builder()
        .header("some-c/gcd.h")
        .use_core()
        .generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    println!("cargo:rerun-if-changed=some-c");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
