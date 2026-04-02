// Tests for Problem 0606: Construct String from Binary Tree
// Java reference: src/test/java/g0601_0700/s0606_construct_string_from_binary_tree/SolutionTest.java

use leetcode_in_rust::s0606::construct_string_from_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_tree2str() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, 4));
    let tree = tree_from_vec(vec![Some(1), Some(2), Some(3), Some(4)]);
    assert_eq!(Solution::tree2str(tree), "1(2(4))(3)");
}

#[test]
fn test_tree2str2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 2, 3, null, 4));
    let tree = tree_from_vec(vec![Some(1), Some(2), Some(3), None, Some(4)]);
    assert_eq!(Solution::tree2str(tree), "1(2()(4))(3)");
}
