extern crate napi_build;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    napi_build::setup();

    let obs_include_dir = env::var("LIBOBS_INCLUDE_DIR").expect("LIBOBS_INCLUDE_DIR is not set");
    let obs_lib_dir = env::var("LIBOBS_LIBRARY_DIR").expect("LIBOBS_LIBRARY_DIR is not set");
    let obs_header = Path::new(&obs_include_dir)
        .join("obs.h")
        .to_string_lossy()
        .to_string();

    println!("cargo:rustc-link-search={}", obs_lib_dir);
    // Link against obs.(lib|so|dylib)
    println!("cargo:rustc-link-lib=obs");
    println!("cargo:rerun-if-changed={}", obs_header);

    let bindings = bindgen::Builder::default()
        .header(obs_header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
