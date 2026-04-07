// Tests for Problem 2156: Find Substring With Given Hash Value
// Java reference: src/test/java/g2101_2200/s2156_find_substring_with_given_hash_value/SolutionTest.java

use leetcode_in_rust::s2156::find_substring_with_given_hash_value::Solution;

#[test]
fn test_sub_str_hash() {
    assert_eq!(
        Solution::sub_str_hash("leetcode".to_string(), 7, 20, 2, 0),
        "ee"
    );
}

#[test]
fn test_sub_str_hash2() {
    assert_eq!(
        Solution::sub_str_hash("fbxzaad".to_string(), 31, 100, 3, 32),
        "fbx"
    );
}
