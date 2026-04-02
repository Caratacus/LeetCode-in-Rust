// Tests for Problem 0273: Integer to English Words
// Java reference: src/test/java/g0201_0300/s0273_integer_to_english_words/SolutionTest.java

use leetcode_in_rust::s0273::integer_to_english_words::Solution;

#[test]
fn test_number_to_words() {
    assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
}

#[test]
fn test_number_to_words2() {
    assert_eq!(Solution::number_to_words(12345), "Twelve Thousand Three Hundred Forty Five");
}

#[test]
fn test_number_to_words3() {
    assert_eq!(
        Solution::number_to_words(1234567),
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    );
}
