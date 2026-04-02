// Tests for Problem 1283: Find the Smallest Divisor Given a Threshold
// Java reference: src/test/java/g1201_1300/s1283_find_the_smallest_divisor_given_a_threshold/SolutionTest.java

use leetcode_in_rust::s1283::find_the_smallest_divisor_given_a_threshold::Solution;

#[test]
fn test_smallest_divisor() {
    assert_eq!(Solution::smallest_divisor(vec![1, 2, 5, 9], 6), 5);
}

#[test]
fn test_smallest_divisor2() {
    assert_eq!(Solution::smallest_divisor(vec![44, 22, 33, 11, 1], 5), 44);
}
