// Tests for Problem 2063: Vowels of All Substrings
// Java reference: src/test/java/g2001_2100/s2063_vowels_of_all_substrings/SolutionTest.java

use leetcode_in_rust::s2063::vowels_of_all_substrings::Solution;

#[test]
fn test_count_vowels() {
    assert_eq!(Solution::count_vowels("aba".to_string()), 6);
}

#[test]
fn test_count_vowels2() {
    assert_eq!(Solution::count_vowels("abc".to_string()), 3);
}

#[test]
fn test_count_vowels3() {
    assert_eq!(Solution::count_vowels("ltcd".to_string()), 0);
}
