fn main() {
    // Set the environment variables for OpenSSL
    println!("cargo:rustc-link-search=native=vendor/openssl/installation/lib");
    println!("cargo:rustc-link-lib=static=ssl");
    println!("cargo:rustc-link-lib=static=crypto");
}
