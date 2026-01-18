# Cross compile a Rust project with C dependencies into RISC-V 

## Generate Rust bindings for the C code 

The Rust FFI bindings to the C function are generated at build time in `build.rs` with

```rust
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
```
and then `include!(concat!(env!("OUT_DIR"), "/bindings.rs"))`.

but they can also be generated manually

```rust
#[repr(C)]
pub struct Pair {
    pub n: ::core::ffi::c_int,
    pub m: ::core::ffi::c_int,
}

unsafe extern "C" {
    pub fn gcd(ps: *mut Pair) -> ::core::ffi::c_int;
}
```

or using [rust-bindgen](https://github.com/rust-lang/rust-bindgen) CLI, eg. `bindgen some-c-code/gcd.h -o src/bindings.rs`

## Cross compilation to RISC-V (riscv64gc-unknown-linux-musl)

Install a RISC-V musl cross-compiler from [musl.cc](https://musl.cc) 

> [!note] 
> it is not available in Arch or AUR, so manual install is required
> ```bash
> cd /opt
> sudo wget https://musl.cc/riscv64-linux-musl-cross.tgz
> sudo tar xzf riscv64-linux-musl-cross.tgz
> # Add to PATH in your shell config:
> export PATH="/opt/riscv64-linux-musl-cross/bin:$PATH"
> ```

and then create a `.cargo/config.toml` file and setup the RISC-V toolchain

```toml
[target.riscv64gc-unknown-linux-musl]
linker = "riscv64-linux-musl-gcc"
rustflags = ["-C", "target-feature=+crt-static"]

[env]
CC_riscv64gc_unknown_linux_musl = "riscv64-linux-musl-gcc"
AR_riscv64gc_unknown_linux_musl = "riscv64-linux-musl-ar"
```

add the right target `rustup target add riscv64gc-unknown-linux-musl` then build normally with `cargo build --target riscv64gc-unknown-linux-musl`

Alternatively without the `.cargo/config.toml` file, using [Zig](https://ziglang.org/) and [`cargo-zigbuild`](https://github.com/rust-cross/cargo-zigbuild) with `cargo zigbuild --target riscv64gc-unknown-linux-musl`

## Run the tests with QEMU

Install QEMU usermode, `sudo pacman -S qemu-user` configure it as a runner in `.cargo/config.toml`

```toml
[target.riscv64gc-unknown-linux-musl]
runner = "qemu-riscv64"
```

then `cargo test --target riscv64gc-unknown-linux-musl`


## Set default target

Optionally set RISC-V as the default target

```toml
[build]
target = "riscv64gc-unknown-linux-musl"
```

then `cargo build` and `cargo test` will run in RISC-V/QEMU
