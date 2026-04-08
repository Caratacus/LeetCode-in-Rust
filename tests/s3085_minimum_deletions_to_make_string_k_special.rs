// Tests for Problem 3085: Minimum Deletions to Make String K-Special
// Java reference: src/test/java/g3001_3100/s3085_minimum_deletions_to_make_string_k_special/SolutionTest.java

use leetcode_in_rust::s3085::minimum_deletions_to_make_string_k_special::Solution;

#[test]
fn test_minimum_deletions() {
    assert_eq!(Solution::minimum_deletions("aabcaba".to_string(), 0), 3);
}

#[test]
fn test_minimum_deletions2() {
    assert_eq!(Solution::minimum_deletions("dabdcbdcdcd".to_string(), 2), 2);
}

#[test]
fn test_minimum_deletions3() {
    assert_eq!(Solution::minimum_deletions("aaabaaa".to_string(), 2), 1);
}
