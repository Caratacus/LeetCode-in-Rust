// Tests for Problem 2398: Maximum Number of Robots Within Budget
// Java reference: src/test/java/g2301_2400/s2398_maximum_number_of_robots_within_budget/SolutionTest.java

use leetcode_in_rust::s2398::maximum_number_of_robots_within_budget::Solution;

#[test]
fn test_maximum_robots() {
    assert_eq!(
        Solution::maximum_robots(vec![3, 6, 1, 3, 4], vec![2, 1, 3, 4, 5], 25),
        3
    );
}

#[test]
fn test_maximum_robots2() {
    assert_eq!(Solution::maximum_robots(vec![11, 12, 19], vec![10, 8, 7], 19), 0);
}
