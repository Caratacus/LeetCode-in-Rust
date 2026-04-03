// Tests for Problem 1721: Swapping Nodes in a Linked List
// Java reference: src/test/java/g1701_1800/s1721_swapping_nodes_in_a_linked_list/SolutionTest.java

use leetcode_in_rust::s1721::swapping_nodes_in_a_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_swap_nodes() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::swap_nodes(head, 2);
    assert_eq!(linked_list_to_vec(result), vec![1, 4, 3, 2, 5]);
}

#[test]
fn test_swap_nodes2() {
    let head = linked_list_from_vec(vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5]);
    let result = Solution::swap_nodes(head, 5);
    assert_eq!(
        linked_list_to_vec(result),
        vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5]
    );
}
