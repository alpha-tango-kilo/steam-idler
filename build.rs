fn main() {
    if cfg!(feature = "docker") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/lib");
    }
}
