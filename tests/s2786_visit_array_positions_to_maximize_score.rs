// Tests for Problem 2786: Visit Array Positions to Maximize Score
// Java reference: src/test/java/g2701_2800/s2786_visit_array_positions_to_maximize_score/SolutionTest.java

use leetcode_in_rust::s2786::visit_array_positions_to_maximize_score::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(vec![2, 4, 6, 8], 3), 20);
}
