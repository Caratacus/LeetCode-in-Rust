// Tests for Problem 2074: Reverse Nodes in Even Length Groups
// Java reference: src/test/java/g2001_2100/s2074_reverse_nodes_in_even_length_groups/SolutionTest.java

use leetcode_in_rust::s2074::reverse_nodes_in_even_length_groups::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_reverse_even_length_groups() {
    let head = linked_list_from_vec(vec![5, 2, 6, 3, 9, 1, 7, 3, 8, 4]);
    let result = Solution::reverse_even_length_groups(head);
    assert_eq!(linked_list_to_vec(result), vec![5, 6, 2, 3, 9, 1, 4, 8, 3, 7]);
}

#[test]
fn test_reverse_even_length_groups2() {
    let head = linked_list_from_vec(vec![1, 1, 0, 6]);
    let result = Solution::reverse_even_length_groups(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 0, 1, 6]);
}

#[test]
fn test_reverse_even_length_groups3() {
    let head = linked_list_from_vec(vec![1, 1, 0, 6, 5]);
    let result = Solution::reverse_even_length_groups(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 0, 1, 5, 6]);
}
