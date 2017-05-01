#![feature(no_core)]
#![no_core]

extern crate libcore_mini as core;

// These are imported to get for-loops working
#[allow(unused_imports)]
use core::option;
#[allow(unused_imports)]
use core::iter;
#[allow(unused_imports)]
use core::ops;

pub mod prelude;
pub mod opcodes;
pub mod peripherals;
pub mod machine;
