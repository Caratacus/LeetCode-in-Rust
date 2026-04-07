// Tests for Problem 1833: Maximum Ice Cream Bars
// Java reference: src/test/java/g1801_1900/s1833_maximum_ice_cream_bars/SolutionTest.java

use leetcode_in_rust::s1833::maximum_ice_cream_bars::Solution;

#[test]
fn test_max_ice_cream() {
    assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
}

#[test]
fn test_max_ice_cream2() {
    assert_eq!(Solution::max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
}

#[test]
fn test_max_ice_cream3() {
    assert_eq!(Solution::max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
}
