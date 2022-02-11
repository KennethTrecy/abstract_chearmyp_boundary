#![cfg_attr(feature = "no_std", no_std)]

//! # Abstract Chearmyp Boundary
//! Please read the README.md for more information.
//!
//! ## Features available
//!- `no_std`: Uses the `core` crate instead of `std` crate.
#![cfg_attr(
	feature = "no_std",
	doc = "- `range_boundary`: Implements [AbstractBoundary] for [core::ops::Range].",
)]
#![cfg_attr(
	not(feature = "no_std"),
	doc = "- `range_boundary`: Implements [AbstractBoundary] for [std::ops::Range].",
)]
#![cfg_attr(
	feature = "no_std",
	doc = "- `vec_boundary_collection`: Implements [AbstractBoundaryCollection] for [alloc::vec::Vec].",
)]
#![cfg_attr(
	not(feature = "no_std"),
	doc = "- `vec_boundary_collection`: Implements [AbstractBoundaryCollection] for [Vec].",
)]

#[cfg(feature = "no_std")]
#[macro_use]
extern crate alloc;

#[cfg(any(feature = "range_boundary", feature = "vec_boundary_collection"))]
mod native {
	#[cfg(feature = "no_std")]
	pub use core::{
		ops::Range,
		cmp::PartialOrd
	};

	#[cfg(feature = "no_std")]
	pub use alloc::vec::Vec;

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		vec::Vec,
		ops::Range,
		cmp::PartialOrd
	};
}

mod abstract_boundary;
mod abstract_boundary_collection;

pub use abstract_boundary::AbstractBoundary;
pub use abstract_boundary_collection::AbstractBoundaryCollection;
