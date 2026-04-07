// Tests for Problem 1882: Process Tasks Using Servers
// Java reference: src/test/java/g1801_1900/s1882_process_tasks_using_servers/SolutionTest.java

use leetcode_in_rust::s1882::process_tasks_using_servers::Solution;

#[test]
fn test_assign_tasks() {
    assert_eq!(
        Solution::assign_tasks(vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2]),
        vec![2, 2, 0, 2, 1, 2]
    );
}

#[test]
fn test_assign_tasks2() {
    assert_eq!(
        Solution::assign_tasks(vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1]),
        vec![1, 4, 1, 4, 1, 3, 2]
    );
}

#[test]
fn test_assign_tasks3() {
    assert_eq!(Solution::assign_tasks(vec![1], vec![1, 2, 3]), vec![0, 0, 0]);
}
