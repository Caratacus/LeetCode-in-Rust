// Tests for Problem 2095: Delete the Middle Node of a Linked List
// Java reference: src/test/java/g2001_2100/s2095_delete_the_middle_node_of_a_linked_list/SolutionTest.java

use leetcode_in_rust::s2095::delete_the_middle_node_of_a_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_delete_middle() {
    let head = linked_list_from_vec(vec![1, 3, 4, 7, 1, 2, 6]);
    let result = Solution::delete_middle(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 3, 4, 1, 2, 6]);
}

#[test]
fn test_delete_middle2() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4]);
    let result = Solution::delete_middle(head);
    assert_eq!(linked_list_to_vec(result), vec![1, 2, 4]);
}

#[test]
fn test_delete_middle3() {
    let head = linked_list_from_vec(vec![2, 1]);
    let result = Solution::delete_middle(head);
    assert_eq!(linked_list_to_vec(result), vec![2]);
}
