use crate::util::list_node::{ListNode, ListMethods};

/**
Difficulty: [Easy](https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/)

Removes duplicate elements from a sorted linked list.

# Arguments

* `head` - An option that contains the head of the linked list wrapped in a
		   Box. It might be None indicating the list is empty.

# Returns

* An `Option<Box<ListNode>>` which is the head of the modified linked list
  with duplicates removed.

# Examples

```
// You could include an example here demonstrating the usage of the function
`````
*/
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut head = head;
	let mut ptr = head.as_mut();

	while let Some(node) = ptr {
		ptr = match node.next.as_ref().is_some_and(|next| next.val == node.val)
		{
			true => node.delete(),

			false => node.next.as_mut(),
		}
	}

	head
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_list(values: Vec<i32>) -> Option<Box<ListNode>> {
		let mut head = None;
		let current = &mut head;

		for &val in values.iter().rev() {
			let node = Box::new(ListNode {
				val,
				next: current.take(),
			});

			*current = Some(node);
		}

		head
	}

	fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
		let mut values = Vec::new();
		while let Some(node) = head {
			values.push(node.val);
			head = node.next;
		}
		values
	}

	#[test]
	fn test_delete_duplicates() {
		let tests = vec![
			(vec![1, 2, 3, 3, 4], vec![1, 2, 3, 4]),
			(vec![1, 1, 1, 1, 1], vec![1]),
			(vec![1, 2, 2, 3, 4, 4, 5], vec![1, 2, 3, 4, 5]),
			(vec![], vec![]),
		];

		for (input, expected) in tests {
			let list = create_list(input);
			let result = delete_duplicates(list);
			assert_eq!(list_to_vec(result), expected);
		}
	}
}
