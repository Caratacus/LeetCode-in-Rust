// Tests for Problem 1335: Minimum Difficulty of a Job Schedule
// Java reference: src/test/java/g1301_1400/s1335_minimum_difficulty_of_a_job_schedule/SolutionTest.java

use leetcode_in_rust::s1335::minimum_difficulty_of_a_job_schedule::Solution;

#[test]
fn test_min_difficulty() {
    let result = Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2);
    assert_eq!(result, 7);
}

#[test]
fn test_min_difficulty2() {
    let result = Solution::min_difficulty(vec![9, 9, 9], 4);
    assert_eq!(result, -1);
}

#[test]
fn test_min_difficulty3() {
    let result = Solution::min_difficulty(vec![1, 1, 1], 3);
    assert_eq!(result, 3);
}
