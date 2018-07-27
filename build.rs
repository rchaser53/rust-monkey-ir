extern crate gcc;

fn main() {
    println!("cargo:rustc-link-lib=dylib={}", "ffi");

    gcc::Config::new()
        .file("src/static-library.c")
        .include("src")
        .compile("libstatic-library.a");
}