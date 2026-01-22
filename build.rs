use std::path::PathBuf;

fn build_grammar(dir: &PathBuf, lib_name: &str) {
    let mut build = cc::Build::new();
    build.include(dir).file(dir.join("parser.c"));

    // Include external scanner if present (required by some grammars)
    let scanner_c = dir.join("scanner.c");
    if scanner_c.exists() {
        build.file(scanner_c.clone());
    }

    // Suppress the specific warning from vendored tree-sitter code
    if build.get_compiler().is_like_clang() || build.get_compiler().is_like_gnu() {
        build.flag("-Wno-unused-but-set-variable");
    }

    // Compile to object file and link directly
    build.compile(lib_name);

    // Link the static library explicitly
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-arg={out_dir}/lib{lib_name}.a");

    // Tell cargo to rerun if the parser/scanner source changes
    println!("cargo:rerun-if-changed={}", dir.join("parser.c").display());
    let scanner_c = dir.join("scanner.c");
    if scanner_c.exists() {
        println!("cargo:rerun-if-changed={}", scanner_c.display());
    }
}

fn main() {
    // Override target-related environment variables to align with Nix expectations
    // The issue is cc crate converts aarch64-apple-darwin -> arm64-apple-macosx
    // but Nix expects arm64-apple-darwin
    unsafe {
        if let Ok(target) = std::env::var("TARGET")
            && target == "aarch64-apple-darwin"
        {
            // Force cc crate to use the darwin naming that Nix expects
            std::env::set_var(
                "CC_aarch64_apple_darwin",
                std::env::var("CC").unwrap_or_else(|_| "clang".to_string()),
            );
            std::env::set_var("CFLAGS_aarch64_apple_darwin", "-target arm64-apple-darwin");
        }
    }

    // Build FunC grammar
    let func_dir: PathBuf = ["grammars", "func", "src"].iter().collect();
    build_grammar(&func_dir, "tree-sitter-func");

    // Build Tact grammar
    let tact_dir: PathBuf = ["grammars", "tact", "src"].iter().collect();
    build_grammar(&tact_dir, "tree-sitter-tact");

    // Build Tolk grammar
    let tolk_dir: PathBuf = ["grammars", "tolk", "src"].iter().collect();
    build_grammar(&tolk_dir, "tree-sitter-tolk");
}
