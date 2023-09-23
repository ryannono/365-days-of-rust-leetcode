#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

trait ListSwaps<T> {
	fn swap_with_nth_node_after(&mut self, n: i32);
	fn swap_with_next(&mut self);
	fn swap_with_end(&mut self);
	fn move_to_end(&mut self);
	fn reverse_from(&mut self);
}

/// A singly-linked list node.
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
	fn new(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		Some(Box::new(ListNode { next, val }))
	}
}

/// Provides operations to swap nodes in the list.
impl ListSwaps<Self> for Box<ListNode> {
	/// Swaps the current node's value and next pointer with its subsequent node.
	///
	/// # Panics
	///
	/// Panics if the current node doesn't have a next node.
	fn swap_with_next(&mut self) {
		let mut next = self.next.take().unwrap();
		let next_next = next.next.take();

		std::mem::swap(self, &mut next);

		next.next = next_next;
		self.next = Some(next);
	}

	/// Swaps the current node with the nth subsequent node in the list.
	///
	/// # Arguments
	///
	/// * `n` - The number of nodes to skip before swapping.
	///
	/// # Safety
	///
	/// Calls to an `unsafe` function to retrieve the nth node. Ensure list
	/// structure remains consistent when using this method.
	fn swap_with_nth_node_after(&mut self, n: i32) {
		if n == 0 || self.next.is_none() {
			return;
		}

		if n == 1 {
			return self.swap_with_next();
		}

		if let Some(swap_node) = unsafe { get_nth_node(self, n) } {
			let head_next = self.next.take();
			let swap_node_next = swap_node.next.take();
			println!("{:?}", swap_node_next);

			std::mem::swap(self, swap_node);

			self.next = head_next;
			swap_node.next = swap_node_next;
		}
	}

	/// Swaps the current node with the last node in the list.
	///
	/// # Safety
	///
	/// Calls to an `unsafe` function to retrieve the last node. Ensure list
	/// structure remains consistent when using this method.
	fn swap_with_end(&mut self) {
		let tail = unsafe { get_last_node(self) };

		let head_next = self.next.take();

		std::mem::swap(tail, self);

		self.next = head_next;
	}

	/// Moves the current node to the end of the list.
	///
	/// This is done by recursively swapping the current node with the next node
	/// until it reaches the end.
	fn move_to_end(&mut self) {
		if self.next.is_none() {
			return;
		}

		self.swap_with_next();
		return self.next.as_mut().unwrap().move_to_end();
	}

	/// Reverses the linked list starting from the current node.
	///
	/// This method reverses all nodes succeeding the current node, making
	/// the current node the new tail of the reversed sublist.
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
}

unsafe fn get_nth_node<'a>(
	mut raw_ptr: *mut Box<ListNode>,
	n: i32,
) -> Option<&'a mut Box<ListNode>> {
	let mut traversed_nodes_count = 0;

	while (*raw_ptr).next.is_some() && traversed_nodes_count < n {
		let next: *mut Box<ListNode> = (*raw_ptr).next.as_mut().unwrap();
		raw_ptr = next;
		traversed_nodes_count += 1;
	}

	match traversed_nodes_count == n {
		true => Some(&mut *raw_ptr),
		false => None,
	}
}

unsafe fn get_last_node<'a>(
	mut raw_ptr: *mut Box<ListNode>,
) -> &'a mut Box<ListNode> {
	while (*raw_ptr).next.is_some() {
		let next: *mut Box<ListNode> = (*raw_ptr).next.as_mut().unwrap();
		raw_ptr = next;
	}

	&mut *raw_ptr
}

#[cfg(test)]
mod tests {
	use crate::list_node::ListSwaps;

	use super::ListNode;

	fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
		let mut new_node = None;
		for &val in vals.iter().rev() {
			new_node = ListNode::new(val, new_node);
		}
		new_node
	}

	fn list_to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
		let mut vec = Vec::new();
		let mut current = node;
		while let Some(mut boxed_node) = current {
			vec.push(boxed_node.val);
			current = boxed_node.next.take();
		}
		vec
	}

	#[test]
	fn test() {
		let mut list =
			create_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).unwrap();

		list.reverse_from();

		let result = list_to_vec(Some(list));

		println!("{:?}", result);
	}
}
