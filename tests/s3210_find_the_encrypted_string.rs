// Tests for Problem 3210: Find the Encrypted String
// Java reference: src/test/java/g3201_3300/s3210_find_the_encrypted_string/SolutionTest.java

use leetcode_in_rust::s3210::find_the_encrypted_string::Solution;

#[test]
fn test_get_encrypted_string() {
    assert_eq!(Solution::get_encrypted_string("dart".to_string(), 3), "tdar");
}

#[test]
fn test_get_encrypted_string2() {
    assert_eq!(Solution::get_encrypted_string("aaa".to_string(), 1), "aaa");
}
