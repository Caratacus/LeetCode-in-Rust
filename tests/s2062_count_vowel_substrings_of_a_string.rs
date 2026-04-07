// Tests for Problem 2062: Count Vowel Substrings of a String
// Java reference: src/test/java/g2001_2100/s2062_count_vowel_substrings_of_a_string/SolutionTest.java

use leetcode_in_rust::s2062::count_vowel_substrings_of_a_string::Solution;

#[test]
fn test_count_vowel_substrings() {
    assert_eq!(Solution::count_vowel_substrings("aeiouu".to_string()), 2);
}

#[test]
fn test_count_vowel_substrings2() {
    assert_eq!(Solution::count_vowel_substrings("unicornarihan".to_string()), 0);
}

#[test]
fn test_count_vowel_substrings3() {
    assert_eq!(Solution::count_vowel_substrings("cuaieuouac".to_string()), 7);
}
