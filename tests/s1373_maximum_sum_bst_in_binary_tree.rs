// Tests for Problem 1373: Maximum Sum BST in Binary Tree
// Java reference: src/test/java/g1301_1400/s1373_maximum_sum_bst_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s1373::maximum_sum_bst_in_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_max_sum_bst() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 4, 3, 2, 4, 2, 5, null, null, null, null, null, null, 4, 6));
    let tree = tree_from_vec(vec![
        Some(1), Some(4), Some(3), Some(2), Some(4), Some(2), Some(5),
        None, None, None, None, None, None, Some(4), Some(6)
    ]);
    assert_eq!(Solution::max_sum_bst(tree), 20);
}

#[test]
fn test_max_sum_bst2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(4, 3, null, 1, 2));
    let tree = tree_from_vec(vec![Some(4), Some(3), None, Some(1), Some(2)]);
    assert_eq!(Solution::max_sum_bst(tree), 2);
}

#[test]
fn test_max_sum_bst3() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(-4, -2, -5));
    let tree = tree_from_vec(vec![Some(-4), Some(-2), Some(-5)]);
    assert_eq!(Solution::max_sum_bst(tree), 0);
}
