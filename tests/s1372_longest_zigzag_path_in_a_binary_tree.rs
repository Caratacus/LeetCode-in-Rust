// Tests for Problem 1372: Longest ZigZag Path in a Binary Tree
// Java reference: src/test/java/g1301_1400/s1372_longest_zigzag_path_in_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s1372::longest_zigzag_path_in_a_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_longest_zig_zag() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, null, 1, 1, 1, null, null, 1, 1, null, 1, null, null, null, 1));
    let tree = tree_from_vec(vec![
        Some(1), None, Some(1), Some(1), Some(1), None, None,
        Some(1), Some(1), None, Some(1), None, None, None, Some(1)
    ]);
    assert_eq!(Solution::longest_zig_zag(tree), 3);
}

#[test]
fn test_longest_zig_zag2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(1, 1, 1, null, 1, null, null, 1, 1, null, 1));
    let tree = tree_from_vec(vec![
        Some(1), Some(1), Some(1), None, Some(1), None, None,
        Some(1), Some(1), None, Some(1)
    ]);
    assert_eq!(Solution::longest_zig_zag(tree), 4);
}

#[test]
fn test_longest_zig_zag3() {
    // TreeNode treeNode = TreeNode.create(Collections.singletonList(1));
    let tree = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::longest_zig_zag(tree), 0);
}
