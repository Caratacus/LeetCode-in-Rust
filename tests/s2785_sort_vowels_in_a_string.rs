// Tests for Problem 2785: Sort Vowels in a String
// Java reference: src/test/java/g2701_2800/s2785_sort_vowels_in_a_string/SolutionTest.java

use leetcode_in_rust::s2785::sort_vowels_in_a_string::Solution;

#[test]
fn test_sort_vowels() {
    assert_eq!(Solution::sort_vowels("lEetcOde".to_string()), "lEOtcede");
}

#[test]
fn test_sort_vowels2() {
    assert_eq!(Solution::sort_vowels("lYmpH".to_string()), "lYmpH");
}
