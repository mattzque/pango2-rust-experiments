use std::path::PathBuf;
use std::env;
use pkg_config::Config;

fn main() {
    let base_path = env::current_dir().unwrap();
    let target_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let prefix = target_path.join("prefix");

    let pkg_config_path = prefix.join("lib").join("x86_64-linux-gnu").join("pkgconfig");

    println!("cargo:pkgconfig={}", pkg_config_path.to_string_lossy());
    println!("cargo:ldpath={}", pkg_config_path.to_string_lossy());
    println!("cargo:rerun-if-changed=BUILD_ALWAYS");

    // run bindgen to generate bindings.rs
    let wrapper_h_path = base_path.join("src/wrapper.h");
    println!("cargo:rerun-if-changed={}", wrapper_h_path.display());

    let freetype = Config::new().cargo_metadata(false).probe("freetype2").unwrap();
    let harfbuzz = Config::new().cargo_metadata(false).probe("harfbuzz").unwrap();
    let cairo = Config::new().cargo_metadata(false).probe("cairo").unwrap();
    let pango = Config::new().cargo_metadata(false).probe("pango2").unwrap();

    let freetype_includes = freetype.include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()));
    let harfbuzz_includes = harfbuzz.include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()));
    let cairo_includes = cairo.include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()));
    let pango_includes = pango.include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()));

    let bindings_path = base_path.join("src/bindings.rs");
    let bindings = bindgen::Builder::default()
        .clang_args(freetype_includes)
        .clang_args(harfbuzz_includes)
        .clang_args(cairo_includes)
        .clang_args(pango_includes)
        .clang_arg(format!("-L{}", freetype.link_paths[0].to_string_lossy()))
        .clang_arg(format!("-L{}", harfbuzz.link_paths[0].to_string_lossy()))
        .clang_arg(format!("-L{}", cairo.link_paths[0].to_string_lossy()))
        .clang_arg(format!("-L{}", pango.link_paths[0].to_string_lossy()))
        .header(wrapper_h_path.to_string_lossy().to_string())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("pango2_.*")
        .blocklist_type("hb_.*")
        .blocklist_type("cairo_t")
        // .blocklist_var("HB_.*")
        .blocklist_type("FT_.*")
        // use freetype_sys for harfbuzz FT features:
        .raw_line("use cairo_sys::cairo_t;")
        .raw_line("use harfbuzz_sys::*;")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings!");

    for lib in &harfbuzz.libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
    for lib in &cairo.libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
    for lib in &pango.libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    for link_path in &harfbuzz.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.to_string_lossy());
    }
    for link_path in &cairo.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.to_string_lossy());
    }
    for link_path in &pango.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.to_string_lossy());
    }
}
