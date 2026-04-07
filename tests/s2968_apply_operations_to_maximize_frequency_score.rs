// Tests for Problem 2968: Apply Operations to Maximize Frequency Score
// Java reference: src/test/java/g2901_3000/s2968_apply_operations_to_maximize_frequency_score/SolutionTest.java

use leetcode_in_rust::s2968::apply_operations_to_maximize_frequency_score::Solution;

#[test]
fn test_max_frequency_score() {
    assert_eq!(Solution::max_frequency_score(vec![1, 2, 6, 4], 3), 3);
}

#[test]
fn test_max_frequency_score2() {
    assert_eq!(Solution::max_frequency_score(vec![1, 4, 4, 2, 4], 0), 3);
}
