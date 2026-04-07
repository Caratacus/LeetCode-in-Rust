// Tests for Problem 2181: Merge Nodes in Between Zeros
// Java reference: src/test/java/g2101_2200/s2181_merge_nodes_in_between_zeros/SolutionTest.java

use leetcode_in_rust::s2181::merge_nodes_in_between_zeros::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_merge_nodes() {
    let head = linked_list_from_vec(vec![0, 3, 1, 0, 4, 5, 2, 0]);
    let result = Solution::merge_nodes(head);
    assert_eq!(linked_list_to_vec(result), vec![4, 11]);
}

#[test]
fn test_merge_nodes2() {
    let head = linked_list_from_vec(vec![0, 1, 0, 3, 0, 2, 2, 0]);
    let result = Solution::merge_nodes(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 3, 4]);
}
