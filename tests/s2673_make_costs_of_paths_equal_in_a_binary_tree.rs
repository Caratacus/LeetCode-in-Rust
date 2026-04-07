// Tests for Problem 2673: Make Costs of Paths Equal in a Binary Tree
// Java reference: src/test/java/g2601_2700/s2673_make_costs_of_paths_equal_in_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s2673::make_costs_of_paths_equal_in_a_binary_tree::Solution;

#[test]
fn test_min_increments() {
    assert_eq!(
        Solution::min_increments(7, vec![1, 5, 2, 2, 3, 3, 1]),
        6
    );
}

#[test]
fn test_min_increments2() {
    assert_eq!(Solution::min_increments(3, vec![5, 3, 3]), 0);
}
