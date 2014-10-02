#![license = "MIT"]
#![feature(plugin_registrar, managed_boxes)]
#![deny(warnings)]

//! Stainless is a lightweight, unopinionated testing framework for Rust.

extern crate syntax;
extern crate rustc;

use self::describe::describe;
use rustc::plugin;

mod describe;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_macro("describe", describe);
}


