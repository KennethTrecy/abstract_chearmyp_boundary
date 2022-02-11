/// An abstraction of boundary.
pub trait AbstractBoundary<T> {
	/// Creates a boundary slice. Equivalent to `n..m` where `n` is the first argument indicating the
	/// start index and `m` is the second argument indicating the end index (exclusive).
	fn new(_: T, _: T) -> Self;

	/// Gets the start index (inclusive) of the boundary.
	fn start(&self) -> &T;

	/// Gets the end index (inclusive) of the boundary.
	fn end(&self) -> &T;

	/// Check if the boundary still has contents.
	fn is_empty(&self) -> bool;
}

// Range is used as sample element when testing Vec.
#[cfg(any(feature = "range_boundary", all(test, feature = "vec_boundary_collection")))]
mod range;

#[cfg(feature = "vec_boundary_collection")]
mod vec;
