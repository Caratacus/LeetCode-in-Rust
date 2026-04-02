// Tests for Problem 1207: Unique Number of Occurrences
// Java reference: src/test/java/g1201_1300/s1207_unique_number_of_occurrences/SolutionTest.java

use leetcode_in_rust::s1207::unique_number_of_occurrences::Solution;

#[test]
fn test_unique_occurrences() {
    assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
}

#[test]
fn test_unique_occurrences2() {
    assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
}

#[test]
fn test_unique_occurrences3() {
    assert_eq!(
        Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
        true
    );
}
