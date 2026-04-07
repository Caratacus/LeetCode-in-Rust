// Tests for Problem 2487: Remove Nodes From Linked List
// Java reference: src/test/java/g2401_2500/s2487_remove_nodes_from_linked_list/SolutionTest.java

use leetcode_in_rust::s2487::remove_nodes_from_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_remove_nodes() {
    let head = linked_list_from_vec(vec![5, 2, 13, 3, 8]);
    let result = Solution::remove_nodes(head);
    assert_eq!(linked_list_to_vec(result), vec![13, 8]);
}

#[test]
fn test_remove_nodes2() {
    let head = linked_list_from_vec(vec![1, 1, 1, 1]);
    let result = Solution::remove_nodes(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 1, 1, 1]);
}
