fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let crates_dir = format!("{}/../../crates", manifest_dir);

    // 追踪组件库源码变化
    println!("cargo:rerun-if-changed={}/ctrl-core/src", crates_dir);
    println!("cargo:rerun-if-changed={}/ctrl-components/src", crates_dir);
    println!("cargo:rerun-if-changed={}/ctrl/src", crates_dir);
    println!("cargo:rerun-if-changed={}/ctrl-core/Cargo.toml", crates_dir);
    println!("cargo:rerun-if-changed={}/ctrl-components/Cargo.toml", crates_dir);
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
}
