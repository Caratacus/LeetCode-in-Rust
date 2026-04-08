// Tests for Problem 2900: Longest Unequal Adjacent Groups Subsequence I
// Java reference: src/test/java/g2801_2900/s2900_longest_unequal_adjacent_groups_subsequence_i/SolutionTest.java

use leetcode_in_rust::s2900::longest_unequal_adjacent_groups_subsequence_i::Solution;

#[test]
fn test_get_words_in_longest_subsequence() {
    assert_eq!(
        Solution::get_words_in_longest_subsequence(
            3,
            vec!["e".to_string(), "a".to_string(), "b".to_string()],
            vec![0, 0, 1]
        ),
        vec!["e".to_string(), "b".to_string()]
    );
}

#[test]
fn test_get_words_in_longest_subsequence2() {
    assert_eq!(
        Solution::get_words_in_longest_subsequence(
            4,
            vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()],
            vec![1, 0, 1, 1]
        ),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}
