// Tests for Problem 0617: Merge Two Binary Trees
// Java reference: src/test/java/g0601_0700/s0617_merge_two_binary_trees/SolutionTest.java

use leetcode_in_rust::s0617::merge_two_binary_trees::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_merge_trees() {
    // TreeNode root1 = TreeNode.create(Arrays.asList(1, 3, 2, 5));
    // TreeNode root2 = TreeNode.create(Arrays.asList(2, 1, 3, null, 4, null, 7));
    let root1 = tree_from_vec(vec![Some(1), Some(3), Some(2), Some(5)]);
    let root2 = tree_from_vec(vec![
        Some(2),
        Some(1),
        Some(3),
        None,
        Some(4),
        None,
        Some(7),
    ]);

    let merged = Solution::merge_trees(root1, root2);
    let result = tree_to_vec(merged);

    // 期望结果: 3,4,5,5,4,null,7
    assert_eq!(
        result,
        vec![Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)]
    );
}

#[test]
fn test_merge_trees2() {
    // TreeNode root1 = TreeNode.create(Arrays.asList(1));
    // TreeNode root2 = TreeNode.create(Arrays.asList(1, 2));
    let root1 = tree_from_vec(vec![Some(1)]);
    let root2 = tree_from_vec(vec![Some(1), Some(2)]);

    let merged = Solution::merge_trees(root1, root2);
    let result = tree_to_vec(merged);

    // 期望结果: 2,2
    assert_eq!(result, vec![Some(2), Some(2)]);
}
