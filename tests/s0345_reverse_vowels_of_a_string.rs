// Tests for Problem 0345: Reverse Vowels of a String
// Java reference: src/test/java/g0301_0400/s0345_reverse_vowels_of_a_string/SolutionTest.java

use leetcode_in_rust::s0345::reverse_vowels_of_a_string::Solution;

#[test]
fn test_reverse_vowels() {
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle");
}

#[test]
fn test_reverse_vowels2() {
    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede");
}
