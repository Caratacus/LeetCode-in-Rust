// Tests for Problem 1631: Path With Minimum Effort
// Java reference: src/test/java/g1601_1700/s1631_path_with_minimum_effort/SolutionTest.java

use leetcode_in_rust::s1631::path_with_minimum_effort::Solution;

#[test]
fn test_minimum_effort_path() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]]), 2);
}

#[test]
fn test_minimum_effort_path2() {
    assert_eq!(Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]]), 1);
}

#[test]
fn test_minimum_effort_path3() {
    assert_eq!(
        Solution::minimum_effort_path(vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1]
        ]),
        0
    );
}
