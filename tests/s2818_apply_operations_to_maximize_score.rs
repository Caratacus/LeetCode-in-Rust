// Tests for Problem 2818: Apply Operations to Maximize Score
// Java reference: src/test/java/g2801_2900/s2818_apply_operations_to_maximize_score/SolutionTest.java

use leetcode_in_rust::s2818::apply_operations_to_maximize_score::Solution;

#[test]
fn test_maximum_score() {
    assert_eq!(Solution::maximum_score(vec![8, 3, 9, 3, 8], 2), 81);
}

#[test]
fn test_maximum_score2() {
    assert_eq!(Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3), 4788);
}
