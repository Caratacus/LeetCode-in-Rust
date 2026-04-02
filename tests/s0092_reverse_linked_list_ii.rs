// Tests for Problem 0092: Reverse Linked List II
// Java reference: src/test/java/g0001_0100/s0092_reverse_linked_list_ii/SolutionTest.java

use leetcode_in_rust::s0092::reverse_linked_list_ii::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_reverse_between() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::reverse_between(head, 2, 4);
    assert_eq!(linked_list_to_vec(result), vec![1, 4, 3, 2, 5]);
}

#[test]
fn test_reverse_between2() {
    let head = linked_list_from_vec(vec![5]);
    let result = Solution::reverse_between(head, 1, 1);
    assert_eq!(linked_list_to_vec(result), vec![5]);
}
