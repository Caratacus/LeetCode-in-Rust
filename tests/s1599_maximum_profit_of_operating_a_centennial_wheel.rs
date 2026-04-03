// Tests for Problem 1599: Maximum Profit of Operating a Centennial Wheel
// Java reference: src/test/java/g1501_1600/s1599_maximum_profit_of_operating_a_centennial_wheel/SolutionTest.java

use leetcode_in_rust::s1599::maximum_profit_of_operating_a_centennial_wheel::Solution;

#[test]
fn test_min_operations_max_profit() {
    assert_eq!(Solution::min_operations_max_profit(vec![8, 3], 5, 6), 3);
}

#[test]
fn test_min_operations_max_profit2() {
    assert_eq!(Solution::min_operations_max_profit(vec![10, 9, 6], 6, 4), 7);
}

#[test]
fn test_min_operations_max_profit3() {
    assert_eq!(Solution::min_operations_max_profit(vec![3, 4, 0, 5, 1], 1, 92), -1);
}
