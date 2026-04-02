// Tests for Problem 0662: Maximum Width of Binary Tree
// Java reference: src/test/java/g0601_0700/s0662_maximum_width_of_binary_tree/SolutionTest.java

use leetcode_in_rust::s0662::maximum_width_of_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_width_of_binary_tree() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 3, 2, 5, 3, null, 9));
    let tree = tree_from_vec(vec![
        Some(1),
        Some(3),
        Some(2),
        Some(5),
        Some(3),
        None,
        Some(9),
    ]);
    assert_eq!(Solution::width_of_binary_tree(tree), 4);
}

#[test]
fn test_width_of_binary_tree2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 3, null, 5, 3));
    let tree = tree_from_vec(vec![Some(1), Some(3), None, Some(5), Some(3)]);
    assert_eq!(Solution::width_of_binary_tree(tree), 2);
}

#[test]
fn test_width_of_binary_tree3() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 3, 2, 5));
    let tree = tree_from_vec(vec![Some(1), Some(3), Some(2), Some(5)]);
    assert_eq!(Solution::width_of_binary_tree(tree), 2);
}
