// Tests for Problem 0025: Reverse Nodes in k-Group
// Java reference: src/test/java/g0001_0100/s0025_reverse_nodes_in_k_group/SolutionTest.java

use leetcode_in_rust::s0025::reverse_nodes_in_k_group::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_reverse_k_group() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::reverse_k_group(head, 2);
    assert_eq!(linked_list_to_vec(result), vec![2, 1, 4, 3, 5]);
}

#[test]
fn test_reverse_k_group2() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::reverse_k_group(head, 3);
    assert_eq!(linked_list_to_vec(result), vec![3, 2, 1, 4, 5]);
}
