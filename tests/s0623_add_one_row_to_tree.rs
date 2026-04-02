// Tests for Problem 0623: Add One Row to Tree
// Java reference: src/test/java/g0601_0700/s0623_add_one_row_to_tree/SolutionTest.java

use leetcode_in_rust::s0623::add_one_row_to_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_add_one_row() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(4, 2, 6, 3, 1, 5));
    let tree = tree_from_vec(vec![Some(4), Some(2), Some(6), Some(3), Some(1), Some(5)]);

    // TreeNode expected = TreeNode.create(Arrays.asList(4, 1, 1, 2, null, null, 6, 3, 1, 5));
    let result = Solution::add_one_row(tree, 1, 2);
    let result_vec = tree_to_vec(result);

    assert_eq!(
        result_vec,
        vec![
            Some(4),
            Some(1),
            Some(1),
            Some(2),
            None,
            None,
            Some(6),
            Some(3),
            Some(1),
            Some(5)
        ]
    );
}

#[test]
fn test_add_one_row2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(4, 2, null, 3, 1));
    let tree = tree_from_vec(vec![Some(4), Some(2), None, Some(3), Some(1)]);

    // TreeNode expected = TreeNode.create(Arrays.asList(4, 2, null, 1, 1, 3, null, null, 1));
    let result = Solution::add_one_row(tree, 1, 3);
    let result_vec = tree_to_vec(result);

    assert_eq!(
        result_vec,
        vec![
            Some(4),
            Some(2),
            None,
            Some(1),
            Some(1),
            Some(3),
            None,
            None,
            Some(1)
        ]
    );
}
