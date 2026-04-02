// Tests for Problem 0242: Valid Anagram
// Java reference: src/test/java/g0201_0300/s0242_valid_anagram/SolutionTest.java

use leetcode_in_rust::s0242::valid_anagram::Solution;

#[test]
fn test_is_anagram() {
    assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
}

#[test]
fn test_is_anagram2() {
    assert_eq!(Solution::is_anagram("rat".to_string(), "car".to_string()), false);
}
