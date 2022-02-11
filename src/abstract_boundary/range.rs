use crate::native::{Range, PartialOrd};
use crate::AbstractBoundary;

/// This is only available if `range_boundary` feature has been activated.
impl<T> AbstractBoundary<T> for Range<T>
where
	T: PartialOrd {
	fn new(start_index: T, end_index: T) -> Self { start_index..end_index }

	fn start(&self) -> &T { &self.start }

	fn end(&self) -> &T { &self.end }

	fn is_empty(&self) -> bool { Range::is_empty(self) }
}

#[cfg(test)]
mod t {
	use super::{AbstractBoundary, Range};

	#[test]
	fn can_get_start() {
		let boundary = Range::new(0, 1);

		let start_index = boundary.start();

		assert_eq!(start_index, &0);
	}

	#[test]
	fn can_get_end() {
		let boundary = Range::new(0, 1);

		let end_index = boundary.end();

		assert_eq!(end_index, &1);
	}

	#[test]
	fn should_be_empty() {
		let boundary = Range::new(0, 0);

		let is_empty = boundary.is_empty();

		assert_eq!(is_empty, true);
	}

	#[test]
	fn should_not_be_empty() {
		let boundary = Range::new(0, 1);

		let is_empty = boundary.is_empty();

		assert_eq!(is_empty, false);
	}
}
