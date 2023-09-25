#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

impl ListNode {
	/// Creates a new ListNode with a given value and optional next node.
	///
	/// # Arguments
	///
	/// * `val` - The value stored in the new node.
	/// * `next` - An optional reference to the next node.
	///
	/// # Returns
	///
	/// * An `Option` wrapping a boxed `ListNode`.
	#[inline]
	pub fn new(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		Some(Box::new(ListNode { next, val }))
	}

	pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
		let mut new_node = None;
		for &val in vec.iter().rev() {
			new_node = Self::new(val, new_node);
		}

		new_node
	}

	pub fn to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
		let mut vec = Vec::new();
		let mut current = node;
		while let Some(mut boxed_node) = current {
			vec.push(boxed_node.val);
			current = boxed_node.next.take();
		}

		vec
	}
}

pub trait ListMethods<T> {
	fn nth(&mut self, n: i32) -> Option<&mut Box<ListNode>>;
	fn last(&mut self) -> Option<&mut Box<ListNode>>;
	fn swap_with_last(&mut self);
	fn swap_with_next(&mut self) -> Option<&mut Box<ListNode>>;
	fn swap_with_nth(&mut self, n: i32) -> bool;
	fn move_to_end(&mut self);
	fn reverse_from(&mut self);
	fn delete(&mut self) -> Option<&mut Box<ListNode>>;
}

/// A singly-linked list node.
impl ListMethods<Self> for Box<ListNode> {
	fn nth(&mut self, mut n: i32) -> Option<&mut Box<ListNode>> {
		let mut ptr = self;

		while n > 0 {
			ptr = ptr.next.as_mut()?;
			n -= 1;
		}

		Some(ptr)
	}

	fn last(&mut self) -> Option<&mut Box<ListNode>> {
		let mut ptr = self;

		while ptr.next.is_some() {
			ptr = ptr.next.as_mut()?;
		}

		Some(ptr)
	}

	/// Swaps the current node with the last node in the list.
	///
	fn swap_with_last(&mut self) {
		let ptr: *mut Box<ListNode> = self;

		if let Some(tail) = unsafe { (*ptr).last() } {
			let head_next = self.next.take();

			std::mem::swap(tail, self);

			self.next = head_next;
		}
	}

	/// Swaps the current node's value and next pointer with its subsequent node.
	///
	/// # Returns
	///
	/// * The function returns a mutable reference (`if some`) to the node after the swapped pair,
	/// 	which can be useful for iterative pair swapping in a linked list.
	///
	fn swap_with_next(&mut self) -> Option<&mut Box<ListNode>> {
		let mut next = self.next.take()?;
		let next_next = next.next.take();

		std::mem::swap(self, &mut next);

		next.next = next_next;
		self.next = Some(next);

		return self.next.as_mut().and_then(|x| x.next.as_mut());
	}

	/// Swaps the current node with the nth subsequent node in the list.
	///
	/// # Arguments
	///
	/// * `n` - The number of nodes to skip before swapping.
	///
	/// # Returns
	///
	/// * A boolean indicating the success of the swap operation. Returns `true`
	///   if the swap was successful, and `false` otherwise.
	///
	fn swap_with_nth(&mut self, n: i32) -> bool {
		if n == 0 {
			return true;
		}

		if self.next.is_none() {
			return false;
		}

		if n == 1 {
			self.swap_with_next();
			return true;
		}

		let ptr: *mut Box<ListNode> = self;
		if let Some(swap_node) = unsafe { (*ptr).nth(n) } {
			let head_next = self.next.take();
			let swap_node_next = swap_node.next.take();

			std::mem::swap(self, swap_node);

			self.next = head_next;
			swap_node.next = swap_node_next;

			return true;
		}

		false
	}

	/// Moves the current node to the end of the list.
	///
	fn move_to_end(&mut self) {
		if self.next.is_none() {
			return;
		}

		self.swap_with_next();
		return self.next.as_mut().unwrap().move_to_end();
	}

	/// Reverses the linked list starting from the current node.
	///
	fn reverse_from(&mut self) {
		// `previous` initially points to None as we start the reversal.
		// it will hold the reversed sublist - (previously seen nodes)
		let mut previous: Option<Box<ListNode>> = None;

		// Start the reversal from the current node.
		let mut current = ListNode::new(self.val, self.next.take());

		// Loop through the list until the current node is exhausted.
		while let Some(mut current_node) = current {
			// Temporarily store the next node in the original list for continuation.
			let next_node = current_node.next.take();

			// The `current_node` now should point to the `previous` node to achieve the reversal.
			current_node.next = previous.take();

			// Move the `previous` pointer to the current node, effectively
			// making the current node the new head of the reversed sublist.
			previous = Some(current_node);

			// Proceed to the next node in the original list.
			current = next_node;
		}

		// At the end of the loop, `previous` holds the new head of the reversed sublist.
		// We update the current node's values to reflect this change.
		self.val = previous.as_ref().unwrap().val;
		self.next = previous.unwrap().next;
	}

	/// Deletes the current node (`self`) and replaces it with the next node
	/// (`self.next`), effectively skipping over one node in the list.
	///
	/// This function performs an in-place modification of the list.
	///
	fn delete(&mut self) -> Option<&mut Self> {
		match self.next.take() {
			Some(next_node) => {
				*self = next_node;
				Some(self)
			}

			None => self.next.as_mut(),
		}
	}
}

#[cfg(test)]
mod tests {

	use super::{ListMethods, ListNode};

	#[test]
	fn test() {
		let mut list =
			ListNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).unwrap();

		list.delete();

		let result = ListNode::to_vec(Some(list));

		println!("{:?}", result);
	}
}
