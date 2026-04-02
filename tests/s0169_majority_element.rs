// Tests for Problem 0169: Majority Element
// Java reference: src/test/java/g0121_0200/s0169_majority_element/SolutionTest.java

use leetcode_in_rust::s0169::majority_element::Solution;

#[test]
fn test_majority_element() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
}

#[test]
fn test_majority_element2() {
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
