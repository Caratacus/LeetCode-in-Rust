// Tests for Problem 0082: Remove Duplicates from Sorted List II
// Java reference: src/test/java/g0001_0100/s0082_remove_duplicates_from_sorted_list_ii/SolutionTest.java

use leetcode_in_rust::s0082::remove_duplicates_from_sorted_list_ii::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_delete_duplicates() {
    let head = linked_list_from_vec(vec![1, 2, 3, 3, 4, 4, 5]);
    let result = Solution::delete_duplicates(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 5]);
}

#[test]
fn test_delete_duplicates2() {
    let head = linked_list_from_vec(vec![1, 1, 1, 2, 3]);
    let result = Solution::delete_duplicates(head);
    assert_eq!(linked_list_to_vec(result), vec![2, 3]);
}
