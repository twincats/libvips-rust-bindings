// (c) Copyright 2019-2023 OLX
fn main() {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("Unable to get manifest directory");
    let lib_dir = format!(
        "{}/lib",
        manifest_dir
    );

    println!("cargo:rustc-link-lib=libvips");
    println!("cargo:rustc-link-lib=libglib-2.0");
    println!("cargo:rustc-link-lib=libgobject-2.0");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir
    );
}
