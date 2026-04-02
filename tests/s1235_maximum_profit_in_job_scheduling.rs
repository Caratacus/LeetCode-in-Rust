// Tests for Problem 1235: Maximum Profit in Job Scheduling
// Java reference: src/test/java/g1201_1300/s1235_maximum_profit_in_job_scheduling/SolutionTest.java

use leetcode_in_rust::s1235::maximum_profit_in_job_scheduling::Solution;

#[test]
fn test_job_scheduling() {
    assert_eq!(
        Solution::job_scheduling(
            vec![1, 2, 3, 3],
            vec![3, 4, 5, 6],
            vec![50, 10, 40, 70]
        ),
        120
    );
}

#[test]
fn test_job_scheduling2() {
    assert_eq!(
        Solution::job_scheduling(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60]
        ),
        150
    );
}

#[test]
fn test_job_scheduling3() {
    assert_eq!(
        Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
        6
    );
}
