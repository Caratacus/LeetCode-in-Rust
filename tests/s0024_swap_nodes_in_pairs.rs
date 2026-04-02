// Tests for Problem 0024: Swap Nodes in Pairs
// Java reference: src/test/java/g0001_0100/s0024_swap_nodes_in_pairs/SolutionTest.java

use leetcode_in_rust::s0024::swap_nodes_in_pairs::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_swap_pairs() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4]);
    let result = Solution::swap_pairs(head);
    assert_eq!(linked_list_to_vec(result), vec![2, 1, 4, 3]);
}

#[test]
fn test_swap_pairs2() {
    let head = linked_list_from_vec(vec![]);
    let result = Solution::swap_pairs(head);
    assert_eq!(linked_list_to_vec(result), vec![]);
}

#[test]
fn test_swap_pairs3() {
    let head = linked_list_from_vec(vec![1]);
    let result = Solution::swap_pairs(head);
    assert_eq!(linked_list_to_vec(result), vec![1]);
}
