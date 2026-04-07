// Tests for Problem 2047: Number of Valid Words in a Sentence
// Java reference: src/test/java/g2001_2100/s2047_number_of_valid_words_in_a_sentence/SolutionTest.java

use leetcode_in_rust::s2047::number_of_valid_words_in_a_sentence::Solution;

#[test]
fn test_count_valid_words() {
    assert_eq!(Solution::count_valid_words("cat and  dog".to_string()), 3);
}

#[test]
fn test_count_valid_words2() {
    assert_eq!(Solution::count_valid_words("!this  1-s b8d!".to_string()), 0);
}

#[test]
fn test_count_valid_words3() {
    assert_eq!(
        Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()),
        5
    );
}
