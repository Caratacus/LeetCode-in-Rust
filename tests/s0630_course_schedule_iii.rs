// Tests for Problem 0630: Course Schedule III
// Java reference: src/test/java/g0601_0700/s0630_course_schedule_iii/SolutionTest.java

use leetcode_in_rust::s0630::course_schedule_iii::Solution;

#[test]
fn test_schedule_course() {
    let input = vec![vec![100, 200], vec![200, 1300], vec![1000, 1250], vec![2000, 3200]];
    assert_eq!(Solution::schedule_course(input), 3);
}

#[test]
fn test_schedule_course2() {
    assert_eq!(Solution::schedule_course(vec![vec![1, 2]]), 1);
}

#[test]
fn test_schedule_course3() {
    assert_eq!(Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
}

#[test]
fn test_schedule_course4() {
    let input = vec![
        vec![100, 200],
        vec![200, 1300],
        vec![1000, 1250],
        vec![2000, 3200],
        vec![300, 1200],
    ];
    assert_eq!(Solution::schedule_course(input), 4);
}
