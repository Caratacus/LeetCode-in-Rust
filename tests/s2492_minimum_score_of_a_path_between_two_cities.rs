// Tests for Problem 2492: Minimum Score of a Path Between Two Cities
// Java reference: src/test/java/g2401_2500/s2492_minimum_score_of_a_path_between_two_cities/SolutionTest.java

use leetcode_in_rust::s2492::minimum_score_of_a_path_between_two_cities::Solution;

#[test]
fn test_min_score() {
    assert_eq!(
        Solution::min_score(4, vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]),
        5
    );
}

#[test]
fn test_min_score2() {
    assert_eq!(
        Solution::min_score(4, vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 8]]),
        2
    );
}
