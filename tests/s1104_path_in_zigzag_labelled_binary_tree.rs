// Tests for Problem 1104: Path in Zigzag Labelled Binary Tree
// Java reference: src/test/java/g1101_1200/s1104_path_in_zigzag_labelled_binary_tree/SolutionTest.java

use leetcode_in_rust::s1104::path_in_zigzag_labelled_binary_tree::Solution;

#[test]
fn test_path_in_zig_zag_tree() {
    assert_eq!(Solution::path_in_zig_zag_tree(14), vec![1, 3, 4, 14]);
}

#[test]
fn test_path_in_zig_zag_tree2() {
    assert_eq!(Solution::path_in_zig_zag_tree(26), vec![1, 2, 6, 10, 26]);
}
