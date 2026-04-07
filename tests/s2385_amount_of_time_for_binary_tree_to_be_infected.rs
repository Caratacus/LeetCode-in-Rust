// Tests for Problem 2385: Amount of Time for Binary Tree to Be Infected
// Java reference: src/test/java/g2301_2400/s2385_amount_of_time_for_binary_tree_to_be_infected/SolutionTest.java

use leetcode_in_rust::s2385::amount_of_time_for_binary_tree_to_be_infected::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_amount_of_time() {
    // TreeNode.create(Arrays.asList(1, 5, 3, null, 4, 10, 6, 9, 2)), start = 3
    let root = tree_from_vec(vec![
        Some(1), Some(5), Some(3), None, Some(4), Some(10), Some(6), Some(9), Some(2)
    ]);
    assert_eq!(Solution::amount_of_time(root, 3), 4);
}

#[test]
fn test_amount_of_time2() {
    // TreeNode.create(Arrays.asList(1)), start = 1
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::amount_of_time(root, 1), 0);
}

#[test]
fn test_amount_of_time3() {
    // TreeNode.create(Arrays.asList(1, 2, null, 3, null, 4, null, 5)), start = 4
    let root = tree_from_vec(vec![
        Some(1), Some(2), None, Some(3), None, Some(4), None, Some(5)
    ]);
    assert_eq!(Solution::amount_of_time(root, 4), 3);
}
