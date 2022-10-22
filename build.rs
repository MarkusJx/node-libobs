extern crate napi_build;

use libloading_bindgen::{generate_bindings, BindingStrategy};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use syn::ForeignItemFn;
use syn::__private::ToTokens;

struct IncludeStrategy {}

impl IncludeStrategy {
    fn new() -> Self {
        Self {}
    }
}

impl BindingStrategy for IncludeStrategy {
    fn should_include(&self, item: &ForeignItemFn) -> bool {
        item.sig.variadic.is_none()
            && item.sig.ident.to_string().starts_with("obs_")
            && !item.sig.ident.to_string().ends_with("_ui")
    }
}

fn main() {
    napi_build::setup();

    let obs_include_dir = env::var("LIBOBS_INCLUDE_DIR").expect("LIBOBS_INCLUDE_DIR is not set");
    //let obs_lib_dir = env::var("LIBOBS_LIBRARY_DIR").expect("LIBOBS_LIBRARY_DIR is not set");
    let obs_header = Path::new(&obs_include_dir)
        .join("obs.h")
        .to_string_lossy()
        .to_string();

    //println!("cargo:rustc-link-search={}", obs_lib_dir);
    // Link against obs.(lib|so|dylib)
    //println!("cargo:rustc-link-lib=obs");
    println!("cargo:rerun-if-changed={}", obs_header);

    let builder = bindgen::Builder::default()
        .header(obs_header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let file = generate_bindings(builder, &IncludeStrategy::new()).expect("");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    std::fs::write(&out_path, file.to_token_stream().to_string())
        .expect("Unable to write bindings");

    Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(out_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
