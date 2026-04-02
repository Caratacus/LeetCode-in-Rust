// Tests for Problem 1017: Convert to Base -2
// Java reference: src/test/java/g1001_1100/s1017_convert_to_base_2/SolutionTest.java

use leetcode_in_rust::s1017::convert_to_base_2::Solution;

#[test]
fn test_base_neg2() {
    assert_eq!(Solution::base_neg2(2), "110");
}

#[test]
fn test_base_neg22() {
    assert_eq!(Solution::base_neg2(3), "111");
}

#[test]
fn test_base_neg23() {
    assert_eq!(Solution::base_neg2(4), "100");
}
