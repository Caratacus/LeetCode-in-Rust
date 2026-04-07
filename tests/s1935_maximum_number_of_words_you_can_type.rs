// Tests for Problem 1935: Maximum Number of Words You Can Type
// Java reference: src/test/java/g1901_2000/s1935_maximum_number_of_words_you_can_type/SolutionTest.java

use leetcode_in_rust::s1935::maximum_number_of_words_you_can_type::Solution;

#[test]
fn test_can_be_typed_words() {
    assert_eq!(Solution::can_be_typed_words("hello world".to_string(), "ad".to_string()), 1);
}

#[test]
fn test_can_be_typed_words2() {
    assert_eq!(Solution::can_be_typed_words("leet code".to_string(), "lt".to_string()), 1);
}

#[test]
fn test_can_be_typed_words3() {
    assert_eq!(Solution::can_be_typed_words("leet code".to_string(), "e".to_string()), 0);
}
