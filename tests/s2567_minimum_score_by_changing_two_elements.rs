// Tests for Problem 2567: Minimum Score by Changing Two Elements
// Java reference: src/test/java/g2501_2600/s2567_minimum_score_by_changing_two_elements/SolutionTest.java

use leetcode_in_rust::s2567::minimum_score_by_changing_two_elements::Solution;

#[test]
fn test_minimize_sum() {
    assert_eq!(Solution::minimize_sum(vec![1, 4, 3]), 0);
}

#[test]
fn test_minimize_sum2() {
    assert_eq!(Solution::minimize_sum(vec![1, 4, 7, 8, 5]), 3);
}
