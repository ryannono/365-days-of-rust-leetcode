use std::collections::HashSet;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>,
}

/// Removes duplicate values from a singly linked list.
///
/// This function traverses the linked list and uses a hash set to keep track of 
/// the unique values encountered. If it encounters a node with a value that is 
/// already in the hash set, it removes that node from the list.
///
/// # Parameters
///
/// - `head`: An `Option` wrapping a `Box<ListNode>` that represents the head of 
///   the linked list. If `None`, the list is considered empty.
///
/// # Returns
///
/// An `Option` wrapping a `Box<ListNode>` that represents the head of the
/// modified list with duplicates removed.
///
/// # Type
///
/// - `ListNode`: A struct representing a node in the list, which should have 
///   at least a field `val` of type `i32` and a field `next` of type 
///   `Option<Box<ListNode>>`.
///
/// # Example
///
/// ```rust
/// use your_crate_name::ListNode;
/// use your_crate_name::delete_duplicates;
///
/// let list = Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { 
///     val: 2, next: Some(Box::new(ListNode { val: 2, next: None }))) 
/// })));
///
/// let result = delete_duplicates(list);
/// // Here, result would be a list with nodes containing values 1 and 2, with 
/// // the duplicate node with value 2 removed.
/// ```
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut nums: HashSet<i32> = HashSet::new();
	let mut head = head;
	let mut ptr: &mut Option<Box<ListNode>> = &mut head;

	while let Some(node) = ptr {
		nums.insert(node.val);

		{
			while node
				.next
				.as_ref()
				.map_or(false, |next_node| nums.contains(&next_node.val))
			{
				node.next =
					node.next.take().and_then(|next_node| next_node.next);
			}
		}

		ptr = &mut node.next;
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
