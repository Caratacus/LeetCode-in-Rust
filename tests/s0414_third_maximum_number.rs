// Tests for Problem 0414: Third Maximum Number
// Java reference: src/test/java/g0401_0500/s0414_third_maximum_number/SolutionTest.java

use leetcode_in_rust::s0414::third_maximum_number::Solution;

#[test]
fn test_third_max() {
    assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
}

#[test]
fn test_third_max2() {
    assert_eq!(Solution::third_max(vec![1, 2]), 2);
}

#[test]
fn test_third_max3() {
    assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
}
