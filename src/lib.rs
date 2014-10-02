#![license = "MIT"]
#![feature(plugin_registrar)]
#![deny(warnings)]
#![deny(missing_doc)]

//! Stainless is a lightweight, unopinionated testing framework for Rust.

extern crate syntax;
extern crate rustc;

use self::describe::describe;
use rustc::plugin;

mod describe;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_macro("describe", describe);
}


