// Tests for Problem 2365: Task Scheduler II
// Java reference: src/test/java/g2301_2400/s2365_task_scheduler_ii/SolutionTest.java

use leetcode_in_rust::s2365::task_scheduler_ii::Solution;

#[test]
fn test_task_scheduler_ii() {
    assert_eq!(Solution::task_scheduler_ii(vec![1, 2, 1, 2, 3, 1], 3), 9);
}

#[test]
fn test_task_scheduler_ii2() {
    assert_eq!(Solution::task_scheduler_ii(vec![5, 8, 8, 5], 2), 6);
}
