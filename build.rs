extern crate gcc;

fn main() {
    println!("cargo:rustc-link-lib=dylib={}", "ffi");

    gcc::Build::new()
        .file("src/c/static-library.c")
        .include("src")
        .compile("libstatic-library.a");
}