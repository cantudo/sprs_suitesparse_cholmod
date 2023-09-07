fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=blas");
    println!("cargo:rustc-link-lib=lapack");

    if std::env::var_os("CARGO_FEATURE_STATIC").is_some() {
        let path_to_cholmod = std::env::var("DEP_SUITESPARSE_SRC_ROOT").unwrap();
        println!("cargo:rustc-link-search=native={path_to_cholmod}");
        println!("cargo:rustc-link-lib=static=cholmod");
    } else {
        println!("cargo:rustc-link-lib=cholmod");
    }
}
    