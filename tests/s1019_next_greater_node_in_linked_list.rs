// Tests for Problem 1019: Next Greater Node In Linked List
// Java reference: src/test/java/g1001_1100/s1019_next_greater_node_in_linked_list/SolutionTest.java

use leetcode_in_rust::s1019::next_greater_node_in_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_next_larger_nodes() {
    let head = linked_list_from_vec(vec![2, 1, 5]);
    assert_eq!(Solution::next_larger_nodes(head), vec![5, 5, 0]);
}

#[test]
fn test_next_larger_nodes2() {
    let head = linked_list_from_vec(vec![2, 7, 4, 3, 5]);
    assert_eq!(Solution::next_larger_nodes(head), vec![7, 0, 5, 5, 0]);
}
