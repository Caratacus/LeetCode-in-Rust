// Tests for Problem 2196: Create Binary Tree From Descriptions
// Java reference: src/test/java/g2101_2200/s2196_create_binary_tree_from_descriptions/SolutionTest.java

use leetcode_in_rust::s2196::create_binary_tree_from_descriptions::Solution;
use leetcode_in_rust::utils::tree_utils::tree_to_vec;

#[test]
fn test_create_binary_tree() {
    let descriptions = vec![
        vec![20, 15, 1],
        vec![20, 17, 0],
        vec![50, 20, 1],
        vec![50, 80, 0],
        vec![80, 19, 1],
    ];
    let root = Solution::create_binary_tree(descriptions);
    // The tree should have root 50
    assert_eq!(tree_to_vec(root), vec![Some(50), Some(20), Some(80), Some(15), Some(17), Some(19), None]);
}

#[test]
fn test_create_binary_tree2() {
    let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];
    let root = Solution::create_binary_tree(descriptions);
    // The tree should have root 1
    assert_eq!(tree_to_vec(root), vec![Some(1), Some(2), None, Some(3), Some(4)]);
}
