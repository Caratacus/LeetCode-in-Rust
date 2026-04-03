// Tests for Problem 1494: Parallel Courses II
// Java reference: src/test/java/g1401_1500/s1494_parallel_courses_ii/SolutionTest.java

use leetcode_in_rust::s1494::parallel_courses_ii::Solution;

#[test]
fn test_min_number_of_semesters() {
    let dependencies = vec![vec![2, 1], vec![3, 1], vec![1, 4]];
    assert_eq!(Solution::min_number_of_semesters(4, dependencies, 2), 3);
}

#[test]
fn test_min_number_of_semesters2() {
    let dependencies = vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]];
    assert_eq!(Solution::min_number_of_semesters(5, dependencies, 2), 4);
}

#[test]
fn test_min_number_of_semesters3() {
    let dependencies: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::min_number_of_semesters(11, dependencies, 2), 6);
}
