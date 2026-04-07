// Tests for Problem 2071: Maximum Number of Tasks You Can Assign
// Java reference: src/test/java/g2001_2100/s2071_maximum_number_of_tasks_you_can_assign/SolutionTest.java

use leetcode_in_rust::s2071::maximum_number_of_tasks_you_can_assign::Solution;

#[test]
fn test_max_task_assign() {
    assert_eq!(
        Solution::max_task_assign(vec![3, 2, 1], vec![0, 3, 3], 1, 1),
        3
    );
}

#[test]
fn test_max_task_assign2() {
    assert_eq!(
        Solution::max_task_assign(vec![5, 4], vec![0, 0, 0], 1, 5),
        1
    );
}

#[test]
fn test_max_task_assign3() {
    assert_eq!(
        Solution::max_task_assign(vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10),
        2
    );
}
