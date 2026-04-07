// Tests for Problem 2899: Last Visited Integers
// Java reference: src/test/java/g2801_2900/s2899_last_visited_integers/SolutionTest.java

use leetcode_in_rust::s2899::last_visited_integers::Solution;

#[test]
fn test_last_visited_integers() {
    assert_eq!(
        Solution::last_visited_integers(vec!["1".to_string(), "2".to_string(), "prev".to_string(), "prev".to_string(), "prev".to_string()]),
        vec![2, 1, -1]
    );
}

#[test]
fn test_last_visited_integers2() {
    assert_eq!(
        Solution::last_visited_integers(vec!["1".to_string(), "prev".to_string(), "2".to_string(), "prev".to_string(), "prev".to_string()]),
        vec![1, 2, 1]
    );
}
