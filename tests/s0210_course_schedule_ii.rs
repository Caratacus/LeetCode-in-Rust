// Tests for Problem 0210: Course Schedule II
// Java reference: src/test/java/g0201_0300/s0210_course_schedule_ii/SolutionTest.java
// Note: Multiple valid orderings may exist; these tests follow Java expectations

use leetcode_in_rust::s0210::course_schedule_ii::Solution;

#[test]
fn test_find_order() {
    let prerequisites = vec![vec![1, 0]];
    assert_eq!(Solution::find_order(2, prerequisites), vec![0, 1]);
}

#[test]
fn test_find_order2() {
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    assert_eq!(Solution::find_order(4, prerequisites), vec![0, 1, 2, 3]);
}

#[test]
fn test_find_order3() {
    let prerequisites: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::find_order(1, prerequisites), vec![0]);
}
