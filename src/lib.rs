//! Crate `hcl` is a Rust library for working with [HCL](https://github.com/hashicorp/hcl).

#![deny(missing_debug_implementations)]

#[cfg_attr(test, macro_use)] extern crate pest;
#[macro_use] extern crate pest_derive;

mod ast;
mod parse;

pub use ast::*;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("hcl.pest");
