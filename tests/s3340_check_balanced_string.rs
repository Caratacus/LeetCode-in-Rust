// Tests for Problem 3340: Check Balanced String
// Java reference: src/test/java/g3301_3400/s3340_check_balanced_string/SolutionTest.java

use leetcode_in_rust::s3340::check_balanced_string::Solution;

#[test]
fn test_is_balanced() {
    assert_eq!(Solution::is_balanced("1234".to_string()), false);
}

#[test]
fn test_is_balanced2() {
    assert_eq!(Solution::is_balanced("24123".to_string()), true);
}
