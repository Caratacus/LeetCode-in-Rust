// Tests for Problem 1456: Maximum Number of Vowels in a Substring of Given Length
// Java reference: src/test/java/g1401_1500/s1456_maximum_number_of_vowels_in_a_substring_of_given_length/SolutionTest.java

use leetcode_in_rust::s1456::maximum_number_of_vowels_in_a_substring_of_given_length::Solution;

#[test]
fn test_max_vowels() {
    assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
}

#[test]
fn test_max_vowels2() {
    assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
}

#[test]
fn test_max_vowels3() {
    assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
}
