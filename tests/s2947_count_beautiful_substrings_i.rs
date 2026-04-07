// Tests for Problem 2947: Count Beautiful Substrings I
// Java reference: src/test/java/g2901_3000/s2947_count_beautiful_substrings_i/SolutionTest.java

use leetcode_in_rust::s2947::count_beautiful_substrings_i::Solution;

#[test]
fn test_beautiful_substrings() {
    assert_eq!(Solution::beautiful_substrings("baeyh".to_string(), 2), 2);
}

#[test]
fn test_beautiful_substrings2() {
    assert_eq!(Solution::beautiful_substrings("abba".to_string(), 1), 3);
}

#[test]
fn test_beautiful_substrings3() {
    assert_eq!(Solution::beautiful_substrings("bcdf".to_string(), 1), 0);
}
