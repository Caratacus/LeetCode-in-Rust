// Tests for Problem 0393: UTF-8 Validation
// Java reference: src/test/java/g0301_0400/s0393_utf_8_validation/SolutionTest.java

use leetcode_in_rust::s0393::utf_8_validation::Solution;

#[test]
fn test_valid_utf8() {
    assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
}

#[test]
fn test_valid_utf82() {
    assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
}
