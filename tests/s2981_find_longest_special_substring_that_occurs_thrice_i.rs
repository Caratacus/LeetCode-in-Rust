// Tests for Problem 2981: Find Longest Special Substring That Occurs Thrice I
// Java reference: src/test/java/g2901_3000/s2981_find_longest_special_substring_that_occurs_thrice_i/SolutionTest.java

use leetcode_in_rust::s2981::find_longest_special_substring_that_occurs_thrice_i::Solution;

#[test]
fn test_maximum_length() {
    assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
}

#[test]
fn test_maximum_length2() {
    assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
}

#[test]
fn test_maximum_length3() {
    assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
}
