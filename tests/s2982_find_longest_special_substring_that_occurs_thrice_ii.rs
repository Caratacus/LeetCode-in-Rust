// Tests for Problem 2982: Find Longest Special Substring That Occurs Thrice II
// Java reference: src/test/java/g2901_3000/s2982_find_longest_special_substring_that_occurs_thrice_ii/SolutionTest.java

use leetcode_in_rust::s2982::find_longest_special_substring_that_occurs_thrice_ii::Solution;

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
