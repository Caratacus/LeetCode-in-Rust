// Tests for Problem 0621: Task Scheduler
// Java reference: src/test/java/g0601_0700/s0621_task_scheduler/SolutionTest.java

use leetcode_in_rust::s0621::task_scheduler::Solution;

#[test]
fn test_least_interval() {
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
        8
    );
}

#[test]
fn test_least_interval2() {
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
        6
    );
}

#[test]
fn test_least_interval3() {
    assert_eq!(
        Solution::least_interval(
            vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2
        ),
        16
    );
}
