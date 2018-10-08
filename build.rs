extern crate cc;

fn main() {
    println!("cargo:rustc-link-lib=dylib={}", "ffi");
}
