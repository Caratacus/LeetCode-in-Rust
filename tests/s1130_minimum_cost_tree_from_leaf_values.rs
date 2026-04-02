// Tests for Problem 1130: Minimum Cost Tree From Leaf Values
// Java reference: src/test/java/g1101_1200/s1130_minimum_cost_tree_from_leaf_values/SolutionTest.java

use leetcode_in_rust::s1130::minimum_cost_tree_from_leaf_values::Solution;

#[test]
fn test_mct_from_leaf_values() {
    assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
}

#[test]
fn test_mct_from_leaf_values2() {
    assert_eq!(Solution::mct_from_leaf_values(vec![4, 11]), 44);
}
