// Tests for Problem 2383: Minimum Hours of Training to Win a Competition
// Java reference: src/test/java/g2301_2400/s2383_minimum_hours_of_training_to_win_a_competition/SolutionTest.java

use leetcode_in_rust::s2383::minimum_hours_of_training_to_win_a_competition::Solution;

#[test]
fn test_min_number_of_hours() {
    assert_eq!(
        Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
        8
    );
}

#[test]
fn test_min_number_of_hours2() {
    assert_eq!(Solution::min_number_of_hours(2, 4, vec![1], vec![3]), 0);
}
