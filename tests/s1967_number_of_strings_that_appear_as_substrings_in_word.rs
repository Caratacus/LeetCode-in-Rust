// Tests for Problem 1967: Number of Strings That Appear as Substrings in Word
// Java reference: src/test/java/g1901_2000/s1967_number_of_strings_that_appear_as_substrings_in_word/SolutionTest.java

use leetcode_in_rust::s1967::number_of_strings_that_appear_as_substrings_in_word::Solution;

#[test]
fn test_num_of_strings() {
    assert_eq!(
        Solution::num_of_strings(
            vec!["a".to_string(), "abc".to_string(), "bc".to_string(), "d".to_string()],
            "abc".to_string()
        ),
        3
    );
}

#[test]
fn test_num_of_strings2() {
    assert_eq!(
        Solution::num_of_strings(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "aaaaabbbbb".to_string()
        ),
        2
    );
}

#[test]
fn test_num_of_strings3() {
    assert_eq!(
        Solution::num_of_strings(
            vec!["a".to_string(), "a".to_string(), "a".to_string()],
            "ab".to_string()
        ),
        3
    );
}
