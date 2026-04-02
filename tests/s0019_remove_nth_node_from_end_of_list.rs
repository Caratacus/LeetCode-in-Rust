// Tests for Problem 0019: Remove Nth Node From End of List
// Java reference: src/test/java/g0001_0100/s0019_remove_nth_node_from_end_of_list/SolutionTest.java

use leetcode_in_rust::s0019::remove_nth_node_from_end_of_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_remove_nth_from_end() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::remove_nth_from_end(head, 2);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 3, 5]);
}

#[test]
fn test_remove_nth_from_end2() {
    let head = linked_list_from_vec(vec![1]);
    let result = Solution::remove_nth_from_end(head, 1);
    assert_eq!(linked_list_to_vec(result), vec![]);
}

#[test]
fn test_remove_nth_from_end3() {
    let head = linked_list_from_vec(vec![1, 2]);
    let result = Solution::remove_nth_from_end(head, 1);
    assert_eq!(linked_list_to_vec(result), vec![1]);
}
