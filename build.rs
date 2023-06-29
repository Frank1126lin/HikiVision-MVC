#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

fn main() {
    // 告诉 rustc 需要 link
    println!("cargo:rustc-link-lib=MvCameraControl");

    // 告诉 cargo 当 wrapper.h 变化时重新运行
    println!("cargo:rerun-if-changed=wrapper.h");

    // 配置 bindgen，并生成 Bindings 结构
    let bindings = bindgen::Builder::default()
        .header("./MVC/MvCameraControl.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // 生成 Rust 代码
    bindings
        .write_to_file("./src/bind/bindings.rs")
        .expect("Failed to write bindings!");
}
