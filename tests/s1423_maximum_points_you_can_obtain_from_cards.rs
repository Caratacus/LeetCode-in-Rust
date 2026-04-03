// Tests for Problem 1423: Maximum Points You Can Obtain from Cards
// Java reference: src/test/java/g1401_1500/s1423_maximum_points_you_can_obtain_from_cards/SolutionTest.java

use leetcode_in_rust::s1423::maximum_points_you_can_obtain_from_cards::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
}

#[test]
fn test_max_score3() {
    assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
}
