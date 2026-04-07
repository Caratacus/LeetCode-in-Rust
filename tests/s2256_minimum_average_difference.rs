// Tests for Problem 2256: Minimum Average Difference
// Java reference: src/test/java/g2201_2300/s2256_minimum_average_difference/SolutionTest.java

use leetcode_in_rust::s2256::minimum_average_difference::Solution;

#[test]
fn test_minimum_average_difference() {
    assert_eq!(Solution::minimum_average_difference(vec![2, 5, 3, 9, 5, 3]), 3);
}

#[test]
fn test_minimum_average_difference2() {
    assert_eq!(Solution::minimum_average_difference(vec![0]), 0);
}

#[test]
fn test_minimum_average_difference3() {
    assert_eq!(Solution::minimum_average_difference(vec![4, 2, 0]), 2);
}
