// Tests for Problem 1816: Truncate Sentence
// Java reference: src/test/java/g1801_1900/s1816_truncate_sentence/SolutionTest.java

use leetcode_in_rust::s1816::truncate_sentence::Solution;

#[test]
fn test_truncate_sentence() {
    assert_eq!(
        Solution::truncate_sentence("Hello how are you Contestant".to_string(), 4),
        "Hello how are you".to_string()
    );
}

#[test]
fn test_truncate_sentence2() {
    assert_eq!(
        Solution::truncate_sentence("What is the solution to this problem".to_string(), 4),
        "What is the solution".to_string()
    );
}

#[test]
fn test_truncate_sentence3() {
    assert_eq!(
        Solution::truncate_sentence("chopper is not a tanuki".to_string(), 5),
        "chopper is not a tanuki".to_string()
    );
}
