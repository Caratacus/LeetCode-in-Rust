// Tests for Problem 1450: Number of Students Doing Homework at a Given Time
// Java reference: src/test/java/g1401_1500/s1450_number_of_students_doing_homework_at_a_given_time/SolutionTest.java

use leetcode_in_rust::s1450::number_of_students_doing_homework_at_a_given_time::Solution;

#[test]
fn test_busy_student() {
    assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
}

#[test]
fn test_busy_student2() {
    assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
}
