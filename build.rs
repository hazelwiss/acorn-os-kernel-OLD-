fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=targets/**");
    println!("cargo:rerun-if-changed=.cargo/**");
}
