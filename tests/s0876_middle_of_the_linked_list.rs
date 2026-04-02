// Tests for Problem 0876: Middle of the Linked List
// Java reference: src/test/java/g0801_0900/s0876_middle_of_the_linked_list/SolutionTest.java

use leetcode_in_rust::s0876::middle_of_the_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_middle_node() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::middle_node(head);
    let result_vec = linked_list_to_vec(result);
    assert_eq!(result_vec, vec![3, 4, 5]);
}

#[test]
fn test_middle_node2() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5, 6]);
    let result = Solution::middle_node(head);
    let result_vec = linked_list_to_vec(result);
    assert_eq!(result_vec, vec![4, 5, 6]);
}
