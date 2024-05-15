fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if version_check::is_min_version("1.80.0").unwrap_or(false) {
        println!("cargo:rustc-check-cfg=cfg(doc_cfg)");
        println!("cargo:rustc-check-cfg=cfg(proc_macro_span)");
    }

    if version_check::supports_feature("proc_macro_span").unwrap_or(false) {
        println!("cargo:rustc-cfg=proc_macro_span");
    }
}
