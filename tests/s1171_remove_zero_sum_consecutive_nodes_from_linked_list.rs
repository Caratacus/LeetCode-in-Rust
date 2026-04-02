// Tests for Problem 1171: Remove Zero Sum Consecutive Nodes from Linked List
// Java reference: src/test/java/g1101_1200/s1171_remove_zero_sum_consecutive_nodes_from_linked_list/SolutionTest.java

use leetcode_in_rust::s1171::remove_zero_sum_consecutive_nodes_from_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_remove_zero_sum_sublists() {
    let head = linked_list_from_vec(vec![1, 2, -3, 3, 1]);
    let result = Solution::remove_zero_sum_sublists(head);
    assert_eq!(linked_list_to_vec(result), vec![3, 1]);
}

#[test]
fn test_remove_zero_sum_sublists2() {
    let head = linked_list_from_vec(vec![1, 2, 3, -3, 4]);
    let result = Solution::remove_zero_sum_sublists(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 4]);
}

#[test]
fn test_remove_zero_sum_sublists3() {
    let head = linked_list_from_vec(vec![1, 2, 3, -3, -2]);
    let result = Solution::remove_zero_sum_sublists(head);
    assert_eq!(linked_list_to_vec(result), vec![1]);
}
