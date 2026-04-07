// Tests for Problem 1832: Check if the Sentence Is Pangram
// Java reference: src/test/java/g1801_1900/s1832_check_if_the_sentence_is_pangram/SolutionTest.java

use leetcode_in_rust::s1832::check_if_the_sentence_is_pangram::Solution;

#[test]
fn test_check_if_pangram() {
    assert_eq!(
        Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()),
        true
    );
}

#[test]
fn test_check_if_pangram2() {
    assert_eq!(Solution::check_if_pangram("leetcode".to_string()), false);
}
