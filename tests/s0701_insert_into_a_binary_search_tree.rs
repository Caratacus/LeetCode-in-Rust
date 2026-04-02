// Tests for Problem 0701: Insert into a Binary Search Tree
// Java reference: src/test/java/g0701_0800/s0701_insert_into_a_binary_search_tree/SolutionTest.java

use leetcode_in_rust::s0701::insert_into_a_binary_search_tree::Solution;
use leetcode_in_rust::utils::tree_utils::{tree_from_vec, tree_to_vec};

#[test]
fn test_insert_into_bst() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(4, 2, 7, 1, 3));
    let tree = tree_from_vec(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);

    // TreeNode expected = TreeNode.create(Arrays.asList(4, 2, 7, 1, 3, 5));
    let result = Solution::insert_into_bst(tree, 5);
    let result_vec = tree_to_vec(result);

    assert_eq!(
        result_vec,
        vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]
    );
}

#[test]
fn test_insert_into_bst2() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(40, 20, 60, 10, 30, 50, 70));
    let tree = tree_from_vec(vec![
        Some(40),
        Some(20),
        Some(60),
        Some(10),
        Some(30),
        Some(50),
        Some(70),
    ]);

    // TreeNode expected = TreeNode.create(Arrays.asList(40, 20, 60, 10, 30, 50, 70, null, null, 25));
    let result = Solution::insert_into_bst(tree, 25);
    let result_vec = tree_to_vec(result);

    assert_eq!(
        result_vec,
        vec![
            Some(40),
            Some(20),
            Some(60),
            Some(10),
            Some(30),
            Some(50),
            Some(70),
            None,
            None,
            Some(25)
        ]
    );
}

#[test]
fn test_insert_into_bst3() {
    // TreeNode treeNode = TreeNode.create(Arrays.asList(4, 2, 7, 1, 3, null, null, null, null, null, null));
    let tree = tree_from_vec(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);

    // TreeNode expected = TreeNode.create(Arrays.asList(4, 2, 7, 1, 3, 5));
    let result = Solution::insert_into_bst(tree, 5);
    let result_vec = tree_to_vec(result);

    assert_eq!(
        result_vec,
        vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)]
    );
}
