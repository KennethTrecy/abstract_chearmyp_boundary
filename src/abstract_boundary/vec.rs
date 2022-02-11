use crate::native::{Vec, PartialOrd};
use crate::AbstractBoundary;

/// This is only available if `vec_boundary_collection` feature has been activated.
impl<T, U> AbstractBoundary<T> for Vec<U>
where
	T: PartialOrd,
	U: AbstractBoundary<T> {
	fn new(start_index: T, end_index: T) -> Self { vec![ U::new(start_index, end_index) ] }

	fn start(&self) -> &T {
		let mut iterator = self.iter();
		let mut lowest_index = iterator.next().unwrap().start();

		for other_boundary in iterator {
			let start_index = other_boundary.start();
			if start_index < lowest_index {
				lowest_index = start_index;
			}
		}

		lowest_index
	}

	fn end(&self) -> &T {
		let mut iterator = self.iter();
		let mut highest_index = iterator.next().unwrap().end();

		for other_boundary in iterator {
			let end_index = other_boundary.end();
			if end_index > highest_index {
				highest_index = end_index;
			}
		}

		highest_index
	}

	fn is_empty(&self) -> bool {
		for boundary in self.iter() {
			if !boundary.is_empty() {
				return false;
			}
		}

		true
	}
}

#[cfg(test)]
mod t {
	use crate::native::Range;
	use super::{AbstractBoundary, Vec};

	#[test]
	fn can_get_start() {
		let boundary = <Vec<Range<u8>> as AbstractBoundary<u8>>::new(0, 1);

		let start_index = boundary.start();

		assert_eq!(start_index, &0);
	}

	#[test]
	fn can_get_end() {
		let boundary = <Vec<Range<u8>> as AbstractBoundary<u8>>::new(0, 1);

		let end_index = boundary.end();

		assert_eq!(end_index, &1);
	}

	#[test]
	fn should_be_empty() {
		let boundary = <Vec<Range<u8>> as AbstractBoundary<u8>>::new(0, 0);

		let is_empty = AbstractBoundary::is_empty(&boundary);

		assert_eq!(is_empty, true);
	}

	#[test]
	fn should_not_be_empty() {
		let boundary = <Vec<Range<u8>> as AbstractBoundary<u8>>::new(0, 1);

		let is_empty = AbstractBoundary::is_empty(&boundary);

		assert_eq!(is_empty, false);
	}
}
