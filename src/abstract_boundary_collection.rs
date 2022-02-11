use crate::AbstractBoundary;

/// An abstraction of boundary collection.
pub trait AbstractBoundaryCollection<T, U: AbstractBoundary<T>> : AbstractBoundary<T> {
	/// Gets the boundary at the specified index.
	fn get(&self, _: usize) -> Option<&U>;

	/// Add the source boundary into the collection.
	fn add(&mut self, _: U);
}

#[cfg(feature = "vec_boundary_collection")]
mod vec;
