fn main() {
    println!(
        "cargo:rustc-link-arg=--script={}/kernel.ld",
        env!("CARGO_MANIFEST_DIR")
    );
    println!(
        "cargo:rerun-if-changed={}/kernel.ld",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-lib=static=xil");
    println!("cargo:rustc-link-lib=static=xiltimer");
    println!("cargo:rustc-link-lib=static=xilstandalone");
}
