//! Crate for building and parsing AT Commands

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![deny(missing_docs)]

pub mod builder;
pub(crate) mod formatter;
pub mod parser;
pub(crate) mod tuple_concat;
