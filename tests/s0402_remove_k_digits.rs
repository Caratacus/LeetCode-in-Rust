// Tests for Problem 0402: Remove K Digits
// Java reference: src/test/java/g0401_0500/s0402_remove_k_digits/SolutionTest.java

use leetcode_in_rust::s0402::remove_k_digits::Solution;

#[test]
fn test_remove_kdigits() {
    assert_eq!(Solution::remove_kdigits("1432219".to_string(), 3), "1219");
}

#[test]
fn test_remove_kdigits2() {
    assert_eq!(Solution::remove_kdigits("10200".to_string(), 1), "200");
}

#[test]
fn test_remove_kdigits3() {
    assert_eq!(Solution::remove_kdigits("10".to_string(), 2), "0");
}
