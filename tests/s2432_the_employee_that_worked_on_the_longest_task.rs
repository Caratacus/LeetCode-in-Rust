// Tests for Problem 2432: The Employee That Worked on the Longest Task
// Java reference: src/test/java/g2401_2500/s2432_the_employee_that_worked_on_the_longest_task/SolutionTest.java

use leetcode_in_rust::s2432::the_employee_that_worked_on_the_longest_task::Solution;

#[test]
fn test_hardest_worker() {
    assert_eq!(
        Solution::hardest_worker(vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
        1
    );
}

#[test]
fn test_hardest_worker2() {
    assert_eq!(
        Solution::hardest_worker(vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
        3
    );
}

#[test]
fn test_hardest_worker3() {
    assert_eq!(Solution::hardest_worker(vec![vec![0, 10], vec![10, 20]]), 0);
}
