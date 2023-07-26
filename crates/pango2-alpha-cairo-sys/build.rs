use std::env;
use pkg_config::Config;

fn main() {
    let base_path = env::current_dir().unwrap();

    println!("cargo:rerun-if-changed=BUILD_ALWAYS");

    let wrapper_h_path = base_path.join("src/wrapper.h");

    let freetype = Config::new().cargo_metadata(false).probe("freetype2").unwrap();
    let cairo = Config::new().cargo_metadata(false).probe("cairo").unwrap();

    let freetype_includes = freetype.include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()));
    let cairo_includes = cairo.include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()));

    println!("cargo:include={}", cairo.include_paths[0].to_string_lossy());

    let bindings_path = base_path.join("src/bindings.rs");
    let bindings = bindgen::Builder::default()
        .clang_args(freetype_includes)
        .clang_args(cairo_includes)
        .clang_arg(format!("-L{}", freetype.link_paths[0].to_string_lossy()))
        .clang_arg(format!("-L{}", cairo.link_paths[0].to_string_lossy()))
        .header(wrapper_h_path.to_string_lossy().to_string())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings!");

    for lib in &cairo.libs {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }
    for link_path in &cairo.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.to_string_lossy());
    }
}
