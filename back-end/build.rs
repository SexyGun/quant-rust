fn main() {
    pyo3_build_config::add_extension_module_link_args();
    // 这一行代码在 Rust 的构建脚本 build.rs 中使用，用于指定编译器链接库的指令。
    // 它的作用是在编译过程中告诉 Cargo 和 Rust 编译器需要链接某个动态库。
    println!("cargo:rustc-link-lib=dylib=python3.11"); // 确保使用正确的 Python 版本
}