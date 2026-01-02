use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-env-changed=IFB_STATIC_LIBS");
    println!("cargo:rerun-if-env-changed=IFB_STATIC_LIB_DIR");
    println!("cargo:rerun-if-env-changed=IFB_ALLOWLIST");

    let headers = PathBuf::from("src/wrapper.h");
    let allowlist = env::var("IFB_ALLOWLIST").unwrap_or_else(|_| ".*".to_string());

    let mut builder = bindgen::Builder::default()
        .header(headers.to_string_lossy())
        .allowlist_function(&allowlist)
        .allowlist_type(&allowlist)
        .allowlist_var(&allowlist)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    if let Ok(include_dir) = env::var("IFB_INCLUDE_DIR") {
        builder = builder.clang_arg(format!("-I{include_dir}"));
    }

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if let Ok(lib_dir) = env::var("IFB_STATIC_LIB_DIR") {
        println!("cargo:rustc-link-search=native={lib_dir}");
    }

    if let Ok(libs) = env::var("IFB_STATIC_LIBS") {
        for lib in libs.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            println!("cargo:rustc-link-lib=static={lib}");
        }
    }
}
