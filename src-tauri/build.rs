fn main() {
    println!("cargo::rustc-check-cfg=cfg(test)");
    tauri_build::build()
}
