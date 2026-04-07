// Tests for Problem 1986: Minimum Number of Work Sessions to Finish the Tasks
// Java reference: src/test/java/g1901_2000/s1986_minimum_number_of_work_sessions_to_finish_the_tasks/SolutionTest.java

use leetcode_in_rust::s1986::minimum_number_of_work_sessions_to_finish_the_tasks::Solution;

#[test]
fn test_min_sessions() {
    assert_eq!(Solution::min_sessions(vec![1, 2, 3], 3), 2);
}

#[test]
fn test_min_sessions2() {
    assert_eq!(Solution::min_sessions(vec![3, 1, 3, 1, 1], 8), 2);
}

#[test]
fn test_min_sessions3() {
    assert_eq!(Solution::min_sessions(vec![1, 2, 3, 4, 5], 15), 1);
}
