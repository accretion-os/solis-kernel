fn main() {
    println!("cargo:rerun-if-changed=build/linker.ld");
    println!("cargo:rustc-link-arg=-Tbuild/linker.ld");
}
