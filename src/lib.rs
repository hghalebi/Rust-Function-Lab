//! Rust function learning library.
//!
//! This crate turns your provided curriculum into executable, tested modules.

pub mod lessons;

pub use lessons::{
    closures, error_handling, iterators, lesson_core, memoization, ownership, pipeline,
};
