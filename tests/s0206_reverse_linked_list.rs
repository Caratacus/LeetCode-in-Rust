// Tests for Problem 0206: Reverse Linked List
// Java reference: src/test/java/g0201_0300/s0206_reverse_linked_list/SolutionTest.java

use leetcode_in_rust::s0206::reverse_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_reverse_list() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::reverse_list(head);
    assert_eq!(linked_list_to_vec(result), vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_reverse_list2() {
    let head = linked_list_from_vec(vec![1, 2]);
    let result = Solution::reverse_list(head);
    assert_eq!(linked_list_to_vec(result), vec![2, 1]);
}

#[test]
fn test_reverse_list3() {
    let result = Solution::reverse_list(None);
    assert!(result.is_none());
}
