// Tests for Problem 0823: Binary Trees With Factors
// Java reference: src/test/java/g0801_0900/s0823_binary_trees_with_factors/SolutionTest.java

use leetcode_in_rust::s0823::binary_trees_with_factors::Solution;

#[test]
fn test_num_factored_binary_trees() {
    assert_eq!(Solution::num_factored_binary_trees(vec![2, 4]), 3);
}

#[test]
fn test_num_factored_binary_trees2() {
    assert_eq!(Solution::num_factored_binary_trees(vec![2, 4, 5, 10]), 7);
}
