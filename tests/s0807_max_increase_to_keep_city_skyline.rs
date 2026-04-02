// Tests for Problem 0807: Max Increase to Keep City Skyline
// Java reference: src/test/java/g0801_0900/s0807_max_increase_to_keep_city_skyline/SolutionTest.java

use leetcode_in_rust::s0807::max_increase_to_keep_city_skyline::Solution;

#[test]
fn test_max_increase_keeping_skyline() {
    assert_eq!(
        Solution::max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0]
        ]),
        35
    );
}

#[test]
fn test_max_increase_keeping_skyline2() {
    assert_eq!(
        Solution::max_increase_keeping_skyline(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        0
    );
}
