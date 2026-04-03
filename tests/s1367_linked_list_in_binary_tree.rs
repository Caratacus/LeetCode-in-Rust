// Tests for Problem 1367: Linked List in Binary Tree
// Java reference: src/test/java/g1301_1400/s1367_linked_list_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s1367::linked_list_in_binary_tree::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_is_sub_path() {
    let head = linked_list_from_vec(vec![4, 2, 8]);
    let root = tree_from_vec(vec![Some(1), Some(4), Some(4), None, Some(2), Some(2), None, Some(1), None, Some(6), Some(8), None, None, None, None, Some(1), Some(3)]);
    let result = Solution::is_sub_path(head, root);
    assert_eq!(result, true);
}

#[test]
fn test_is_sub_path2() {
    let head = linked_list_from_vec(vec![1, 4, 2, 6]);
    let root = tree_from_vec(vec![Some(1), Some(4), Some(4), None, Some(2), Some(2), None, Some(1), None, Some(6), Some(8), None, None, None, None, Some(1), Some(3)]);
    let result = Solution::is_sub_path(head, root);
    assert_eq!(result, true);
}

#[test]
fn test_is_sub_path3() {
    let head = linked_list_from_vec(vec![1, 4, 2, 6, 8]);
    let root = tree_from_vec(vec![Some(1), Some(4), Some(4), None, Some(2), Some(2), None, Some(1), None, Some(6), Some(8), None, None, None, None, Some(1), Some(3)]);
    let result = Solution::is_sub_path(head, root);
    assert_eq!(result, false);
}
