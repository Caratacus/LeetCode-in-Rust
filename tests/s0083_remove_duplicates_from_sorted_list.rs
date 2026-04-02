// Tests for Problem 0083: Remove Duplicates from Sorted List
// Java reference: src/test/java/g0001_0100/s0083_remove_duplicates_from_sorted_list/SolutionTest.java

use leetcode_in_rust::s0083::remove_duplicates_from_sorted_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_delete_duplicates() {
    let head = linked_list_from_vec(vec![1, 1, 2]);
    let result = Solution::delete_duplicates(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 2]);
}

#[test]
fn test_delete_duplicates2() {
    let head = linked_list_from_vec(vec![1, 1, 2, 3, 3]);
    let result = Solution::delete_duplicates(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 3]);
}
