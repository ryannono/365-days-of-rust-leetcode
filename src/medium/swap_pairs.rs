use crate::util::list_node::{ListNode, ListMethods};

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

	#[test]
	fn test_empty_list() {
		let list = ListNode::from_vec(vec![]);
		let result = swap_pairs(list);
		assert_eq!(ListNode::to_vec(result), vec![]);
	}

	#[test]
	fn test_odd_number_of_nodes() {
		let list = ListNode::from_vec(vec![1, 2, 3]);
		let result = swap_pairs(list);
		assert_eq!(ListNode::to_vec(result), vec![2, 1, 3]);
	}

	#[test]
	fn test_even_number_of_nodes() {
		let list = ListNode::from_vec(vec![1, 2, 3, 4]);
		let result = swap_pairs(list);
		assert_eq!(ListNode::to_vec(result), vec![2, 1, 4, 3]);
	}
}
