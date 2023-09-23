// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

impl ListNode {
	#[inline]
	fn new(val: i32) -> Self {
		ListNode { next: None, val }
	}
}

/// Trait that extends the functionality of ListNode
/// to include swapping a node with its next node.
trait BubbleNode<T> {
	fn swap_with_next(&mut self) -> Option<&mut T>;
}

impl BubbleNode<Self> for Box<ListNode> {
	/// Swaps the current node (`self`) with its next node (`self.next`) in place.
	/// The function returns a mutable reference to the node after the swapped pair,
	/// which can be useful for iterative pair swapping in a linked list.
	///
	/// # Steps
	/// 1. A (self) -> B (next_node) -> C -> ... (start)
	/// 2. A (self); B (next_node) -> C -> ... (after taking the "next" away from self)
	/// 3. A (self) -> C -> ...; B (next_node) (after taking the "next" away from next_node)
	/// 4. B (self); A (next_node) -> C -> ... (after swapping self and next_node)
	/// 5. B (self) -> A (next_node) -> C -> ... (after assignment)
	fn swap_with_next(&mut self) -> Option<&mut Self> {
		if self.next.is_some() {
			let mut next_node = self.next.take()?;
			self.next = next_node.next.take();
			std::mem::swap(self, &mut next_node);
			self.next = Some(next_node);

			return self.next.as_mut().and_then(|x| x.next.as_mut());
		}

		self.next.as_mut()
	}
}

/// Difficulty: [Medium](https://leetcode.com/problems/swap-nodes-in-pairs/description/)
/// Swaps every two adjacent nodes of a linked list.
///
/// # Description
/// This function takes the head of a singly linked list and swaps each pair
/// of adjacent nodes. If the list has an odd number of nodes, the last node
/// is left unchanged. It then returns the new head after swapping.
///
/// # Examples
/// Given the list: 1 -> 2 -> 3 -> 4, the function will return the list:
/// 2 -> 1 -> 4 -> 3.
///
/// # Arguments
/// * `head`: The starting node of the linked list.
///
/// # Returns
/// An `Option<Box<ListNode>>` which is the new head of the modified linked list.
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut head = head;
	let mut ptr = head.as_mut();

	// While there are nodes to consider, swap each pair.
	while let Some(node) = ptr {
		ptr = node.swap_with_next();
	}

	// Return the modified head of the list.
	head
}

#[cfg(test)]
mod tests {
	use super::{swap_pairs, ListNode};

	fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
		let mut current = None;
		for &val in vals.iter().rev() {
			let mut new_node = ListNode::new(val);
			new_node.next = current;
			current = Some(Box::new(new_node));
		}
		current
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
	fn test_empty_list() {
		let list = create_list(vec![]);
		let result = swap_pairs(list);
		assert_eq!(list_to_vec(result), vec![]);
	}

	#[test]
	fn test_odd_number_of_nodes() {
		let list = create_list(vec![1, 2, 3]);
		let result = swap_pairs(list);
		assert_eq!(list_to_vec(result), vec![2, 1, 3]);
	}

	#[test]
	fn test_even_number_of_nodes() {
		let list = create_list(vec![1, 2, 3, 4]);
		let result = swap_pairs(list);
		assert_eq!(list_to_vec(result), vec![2, 1, 4, 3]);
	}
}
