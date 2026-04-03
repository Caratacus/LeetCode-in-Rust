// Tests for Problem 1462: Course Schedule IV
// Java reference: src/test/java/g1401_1500/s1462_course_schedule_iv/SolutionTest.java

use leetcode_in_rust::s1462::course_schedule_iv::Solution;

#[test]
fn test_check_if_prerequisite() {
    let prerequisites = vec![vec![1, 0]];
    let queries = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(Solution::check_if_prerequisite(2, prerequisites, queries), vec![false, true]);
}

#[test]
fn test_check_if_prerequisite2() {
    let prerequisites: Vec<Vec<i32>> = vec![];
    let queries = vec![vec![1, 0], vec![0, 1]];
    assert_eq!(Solution::check_if_prerequisite(2, prerequisites, queries), vec![false, false]);
}

#[test]
fn test_check_if_prerequisite3() {
    let prerequisites = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
    let queries = vec![vec![1, 0], vec![1, 2]];
    assert_eq!(Solution::check_if_prerequisite(3, prerequisites, queries), vec![true, true]);
}
