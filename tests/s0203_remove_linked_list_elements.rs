// Tests for Problem 0203: Remove Linked List Elements
// Java reference: src/test/java/g0201_0300/s0203_remove_linked_list_elements/SolutionTest.java

use leetcode_in_rust::s0203::remove_linked_list_elements::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_remove_elements() {
    let head = linked_list_from_vec(vec![1, 2, 6, 3, 4, 5, 6]);
    let result = Solution::remove_elements(head, 6);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_remove_elements2() {
    let result = Solution::remove_elements(None, 1);
    assert!(result.is_none());
}

#[test]
fn test_remove_elements3() {
    let head = linked_list_from_vec(vec![7, 7, 7, 7]);
    let result = Solution::remove_elements(head, 7);
    assert!(result.is_none());
}
