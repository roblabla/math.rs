//! Software implementations of operations on IEEE 754 floating point numbers in Rust.
//!
//! The implementations are not optimised to use any. The code is supposed to be very well
//! documented (as if!) so one could learn from it.
//!
//! # Recommendations
//!
//! This library is indended to be used with Rust programs on targets where libm is not available
//! or linking to it is undesirable. It does not provide 1-to-1 feature parity with libm. Namely:
//!
//! * Any global state is not updated or used (this includes errors, rounding direction etc);
//! * Several functions are omitted. Currently:
//!   * nan, nanf – reliance on libc functions;
//!   * Functions that take or return `long double` – `long double` is usually not a standard IEEE
//!     754 type.
//!
//! # Usage
//!
//! In order to use this library nothing special needs to be done. Use regular functions from the
//! `std` or `core` and simply link to this library. In case the target platform has no hardware
//! support for some operation, software implementations provided by this library will be used
//! automagically.
// TODO: provide instructions to override default libm link and how to link to this library.

#![crate_name="math"]
#![crate_type="rlib"]

// Since this is package provides very basic operations, our only dependencies will be Rust’s
// libcore.
#![no_std]

// Do not try use library itself for tests. It does not cope well with libstd.
#![cfg(not(test))]

extern crate cty;

// Reexport everything
pub use abs::*;
pub use copysign::*;
pub use round::*;
pub use floor::*;
pub use ceil::*;
pub use trunc::*;
pub use lround::*;
pub use llround::*;

pub use dim::*;
pub use ma::*;
pub use fmod::*;
// pub use remquo::*;

pub use sin::{sin, sinf};
pub use cos::{cos, cosf};
pub use tan::{tan, tanf};
pub use asin::{asin, asinf};
pub use acos::{acos, acosf};

pub use sqrt::*;
pub use exp::*;

pub use logb::*;
pub use ilogb::*;
pub use scalbn::*;
pub use scalbln::*;
pub use modf::*;
pub use nextafter::*;

pub use min::*;
pub use max::*;
pub use hypot::*;

pub use nearbyint::*;
pub use rint::*;
pub use ldexp::*;

// The modules are split and grouped by function class.
//
// Operations on signs:
pub mod abs;
pub mod copysign;
// Rounding operations:
pub mod round;
pub mod floor;
pub mod ceil;
pub mod trunc;
pub mod lround;
pub mod llround;
// Simple arithmetic operations:
pub mod dim;
pub mod ma;                         // Needs tests
pub mod fmod;
// The complex operations such as sqrt, log etc:
pub mod sqrt;
pub mod exp;
// Even more complex – trigonometric – operations:
pub mod sin;
pub mod cos;
pub mod tan;
pub mod asin;
pub mod acos;

// Introspection:
pub mod logb;
pub mod ilogb;
pub mod scalbn;
pub mod scalbln;
pub mod modf;
pub mod nextafter;
// Other:
pub mod min;
pub mod max;
pub mod hypot;
// Compatibility (possibly non-conforming):
pub mod nearbyint;
pub mod rint; // Also provides lrint and llrint
pub mod ldexp;

// Common functionality.
pub mod utils;
