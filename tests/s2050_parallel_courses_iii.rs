// Tests for Problem 2050: Parallel Courses III
// Java reference: src/test/java/g2001_2100/s2050_parallel_courses_iii/SolutionTest.java

use leetcode_in_rust::s2050::parallel_courses_iii::Solution;

#[test]
fn test_minimum_time() {
    assert_eq!(
        Solution::minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]),
        8
    );
}

#[test]
fn test_minimum_time2() {
    assert_eq!(
        Solution::minimum_time(
            5,
            vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
            vec![1, 2, 3, 4, 5]
        ),
        12
    );
}
