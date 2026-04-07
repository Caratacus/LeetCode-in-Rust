// Tests for Problem 2746: Decremental String Concatenation
// Java reference: src/test/java/g2701_2800/s2746_decremental_string_concatenation/SolutionTest.java

use leetcode_in_rust::s2746::decremental_string_concatenation::Solution;

#[test]
fn test_minimize_concatenated_length() {
    assert_eq!(
        Solution::minimize_concatenated_length(vec!["aa".to_string(), "ab".to_string(), "bc".to_string()]),
        4
    );
}

#[test]
fn test_minimize_concatenated_length2() {
    assert_eq!(
        Solution::minimize_concatenated_length(vec!["ab".to_string(), "b".to_string()]),
        2
    );
}

#[test]
fn test_minimize_concatenated_length3() {
    assert_eq!(
        Solution::minimize_concatenated_length(vec!["aaa".to_string(), "c".to_string(), "aba".to_string()]),
        6
    );
}
