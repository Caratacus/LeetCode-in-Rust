// Tests for Problem 1641: Count Sorted Vowel Strings
// Java reference: src/test/java/g1601_1700/s1641_count_sorted_vowel_strings/SolutionTest.java

use leetcode_in_rust::s1641::count_sorted_vowel_strings::Solution;

#[test]
fn test_count_vowel_strings() {
    assert_eq!(Solution::count_vowel_strings(1), 5);
}

#[test]
fn test_count_vowel_strings2() {
    assert_eq!(Solution::count_vowel_strings(2), 15);
}
