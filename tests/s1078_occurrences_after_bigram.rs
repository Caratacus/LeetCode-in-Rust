// Tests for Problem 1078: Occurrences After Bigram
// Java reference: src/test/java/g1001_1100/s1078_occurrences_after_bigram/SolutionTest.java

use leetcode_in_rust::s1078::occurrences_after_bigram::Solution;

#[test]
fn test_find_ocurrences() {
    assert_eq!(
        Solution::find_ocurrences("alice is a good girl she is a good student".to_string(), "a".to_string(), "good".to_string()),
        vec!["girl".to_string(), "student".to_string()]
    );
}

#[test]
fn test_find_ocurrences2() {
    assert_eq!(
        Solution::find_ocurrences("we will we will rock you".to_string(), "we".to_string(), "will".to_string()),
        vec!["we".to_string(), "rock".to_string()]
    );
}
