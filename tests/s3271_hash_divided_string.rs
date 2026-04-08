// Tests for Problem 3271: Hash Divided String
// Java reference: src/test/java/g3201_3300/s3271_hash_divided_string/SolutionTest.java

use leetcode_in_rust::s3271::hash_divided_string::Solution;

#[test]
fn test_string_hash() {
    assert_eq!(Solution::string_hash("abcd".to_string(), 2), "bf");
}

#[test]
fn test_string_hash2() {
    assert_eq!(Solution::string_hash("mxz".to_string(), 3), "i");
}
