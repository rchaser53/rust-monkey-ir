extern crate cc;

fn main() {
    println!("cargo:rustc-link-lib=dylib={}", "ffi");

    cc::Build::new()
        .file("src/c/static-library.c")
        .include("src")
        .compile("libstatic-library.a");
}