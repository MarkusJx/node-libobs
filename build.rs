extern crate napi_build;

use std::env;
use std::path::PathBuf;

fn main() {
    napi_build::setup();

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search=C:/Users/marku/Desktop/obs-studio/build64/libobs/RelWithDebInfo"
    );

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=obs");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=C:/Users/marku/Desktop/obs-studio/libobs/obs.h");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("C:/Users/marku/Desktop/obs-studio/libobs/obs.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
