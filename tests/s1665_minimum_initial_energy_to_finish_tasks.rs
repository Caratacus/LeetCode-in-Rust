// Tests for Problem 1665: Minimum Initial Energy to Finish Tasks
// Java reference: src/test/java/g1601_1700/s1665_minimum_initial_energy_to_finish_tasks/SolutionTest.java

use leetcode_in_rust::s1665::minimum_initial_energy_to_finish_tasks::Solution;

#[test]
fn test_minimum_effort() {
    assert_eq!(Solution::minimum_effort(vec![vec![1, 2], vec![2, 4], vec![4, 8]]), 8);
}

#[test]
fn test_minimum_effort2() {
    assert_eq!(
        Solution::minimum_effort(vec![
            vec![1, 3],
            vec![2, 4],
            vec![10, 11],
            vec![10, 12],
            vec![8, 9]
        ]),
        32
    );
}

#[test]
fn test_minimum_effort3() {
    assert_eq!(
        Solution::minimum_effort(vec![
            vec![1, 7],
            vec![2, 8],
            vec![3, 9],
            vec![4, 10],
            vec![5, 11],
            vec![6, 12]
        ]),
        27
    );
}
