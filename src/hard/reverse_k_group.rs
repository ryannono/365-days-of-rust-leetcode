use crate::util::list_node::{ListNode, ListSwaps};

/// Difficulty: [Hard](https://leetcode.com/problems/reverse-nodes-in-k-group/description/)
/// 
/// Reverses nodes in `k`-node groups within the linked list.
///
/// Given a linked list, this function will reverse the nodes of the list `k`
/// at a time. If the number of nodes is not a multiple of `k` then the
/// remaining nodes at the end will retain their original order.
///
/// # Arguments
///
/// * `head` - The starting node of the linked list.
/// * `k` - The number of nodes to be grouped and reversed.
///
/// # Returns
///
/// * An `Option` wrapping the new head node of the modified linked list.
///
/// # Examples
///
/// If our linked list is `1->2->3->4->5` and `k` is 2, the function will return
/// `2->1->4->3->5`.
///
/// If `k` is 3, it will return `3->2->1->4->5`.
///
/// Note: Your linked list should have a length of at least `k`.
pub fn reverse_k_group(
	mut head: Option<Box<ListNode>>,
	k: i32,
) -> Option<Box<ListNode>> {
	let mut current = &mut head;
	let mut should_swap = true;
	let mut swap_counter = 0;
	let mut curr_position = 0;

	while let Some(node) = current {
		curr_position += 1;

		match should_swap {
			true => {
				if !node.swap_with_nth_node_after(k - 1 - (swap_counter * 2)) {
					break;
				}

				swap_counter += 1;

				if swap_counter == (k / 2) {
					should_swap = false;
				}
			}

			false => {
				if curr_position % k == 0 {
					should_swap = true;
					swap_counter = 0;
				}
			}
		}

		current = &mut node.next;
	}

	head
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_reverse_k_group() {
		// Test when k = 2
		let list = ListNode::from_vec(vec![1, 2, 3, 4]);
		let result = reverse_k_group(list, 2);
		assert_eq!(result, ListNode::from_vec(vec![2, 1, 4, 3]));

		// Test when k = 3
		let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
		let result = reverse_k_group(list, 3);
		assert_eq!(result, ListNode::from_vec(vec![3, 2, 1, 4, 5]));

		// Test when k = 3 but length is not a multiple of k
		let list = ListNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
		let result = reverse_k_group(list, 3);
		assert_eq!(result, ListNode::from_vec(vec![3, 2, 1, 6, 5, 4, 7]));

		// Test when k > length
		let list = ListNode::from_vec(vec![1, 2, 3]);
		let result = reverse_k_group(list, 4);
		assert_eq!(result, ListNode::from_vec(vec![1, 2, 3]));

		// Test when list is empty
		let list: Option<Box<ListNode>> = None;
		let result = reverse_k_group(list, 2);
		assert_eq!(result, None);
	}
}
