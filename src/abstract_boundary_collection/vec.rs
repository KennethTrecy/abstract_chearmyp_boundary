use crate::native::{Vec, PartialOrd};
use crate::{AbstractBoundary, AbstractBoundaryCollection};

/// This is only available if `vec_boundary_collection` feature has been activated.
///
/// It implements [AbstractBoundaryCollection] for alloc::vec::[Vec] instead if `no_std` feature has
/// been activated.
impl<T, U> AbstractBoundaryCollection<T, U> for Vec<U>
where
	T: PartialOrd,
	U: AbstractBoundary<T> {
	fn get(&self, index: usize) -> Option<&U> {
		mod private {
			use super::Vec;

			pub fn get<T>(vector: &Vec<T>, index: usize) -> Option<&T> {
				vector.get(index)
			}
		}

		private::get(self, index)
	}

	fn add(&mut self, source: U) {
		self.push(source);
	}
}

#[cfg(test)]
mod t {
	use super::{Vec, AbstractBoundaryCollection};

	#[test]
	fn can_get_boundary() {
		let collection = vec![0..1];

		let boundary = AbstractBoundaryCollection::get(&collection, 0);

		assert_eq!(boundary, Some(&(0..1)));
	}

	#[test]
	fn can_add_source() {
		let mut collection = Vec::new();

		collection.add(0..1);

		assert_eq!(collection, vec![0..1]);
	}
}
