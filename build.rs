fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if version_check::supports_feature("proc_macro_span").unwrap_or_default() {
        println!("cargo:rustc-cfg=proc_macro_span");
    }
}
