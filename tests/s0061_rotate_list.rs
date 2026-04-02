// Tests for Problem 0061: Rotate List
// Java reference: src/test/java/g0001_0100/s0061_rotate_list/SolutionTest.java

use leetcode_in_rust::s0061::rotate_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_rotate_right() {
    let head = linked_list_from_vec(vec![1, 2, 3, 4, 5]);
    let result = Solution::rotate_right(head, 2);
    assert_eq!(linked_list_to_vec(result), vec![4, 5, 1, 2, 3]);
}

#[test]
fn test_rotate_right2() {
    let head = linked_list_from_vec(vec![0, 1, 2]);
    let result = Solution::rotate_right(head, 4);
    assert_eq!(linked_list_to_vec(result), vec![2, 0, 1]);
}
