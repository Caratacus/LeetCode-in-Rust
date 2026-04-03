// Tests for Problem 0889: Construct Binary Tree from Preorder and Postorder Traversal
// Java reference: src/test/java/g0801_0900/s0889_construct_binary_tree_from_preorder_and_postorder_traversal/SolutionTest.java

use leetcode_in_rust::s0889::construct_binary_tree_from_preorder_and_postorder_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::tree_to_vec;

#[test]
fn test_construct_from_pre_post() {
    let result = Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]);
    // Expected: [1,2,3,4,5,6,7]
    assert_eq!(tree_to_vec(result), vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)]);
}

#[test]
fn test_construct_from_pre_post2() {
    let result = Solution::construct_from_pre_post(vec![1], vec![1]);
    assert_eq!(tree_to_vec(result), vec![Some(1)]);
}
