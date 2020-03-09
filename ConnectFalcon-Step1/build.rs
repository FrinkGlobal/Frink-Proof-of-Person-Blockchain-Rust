extern crate bindgen;
extern crate cc;

use std::path::PathBuf;

fn main() {

    // Generate Rust FFI binding to falcon.h.
    let bindings = bindgen::Builder::default()
        .header("falcon/api.h")
        .header("falcon/falcon.h")
        .header("falcon/rng.h")
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from("src/lib");

    bindings
        .write_to_file(out_path.join("falcon.rs"))
        .expect("Couldn't write bindings!");

    // Compile falcon.c
    cc::Build::new()
        .file("falcon/falcon-enc.c")
        .file("falcon/falcon-fft.c")
        .file("falcon/falcon-keygen.c")
        .file("falcon/falcon-sign.c")
        .file("falcon/falcon-vrfy.c")
        .file("falcon/frng.c")
        .file("falcon/shake.c")
        .file("falcon/nist.c")
        .file("falcon/rng.c")
        .include("C:/Program Files/OpenSSL-Win64/include")
        .compile("falcon");
        
    println!("cargo:rustc-link-search=static=C:/Program Files/OpenSSL-Win64/lib");
    println!("cargo:rustc-link-lib=static=libcrypto");
    println!("cargo:rustc-link-lib=static=libssl");
}