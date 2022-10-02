#![feature(core_c_str)]
#![feature(alloc_c_string)]
#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate strum_macros;
extern crate alloc;
extern crate core;

mod obs;
