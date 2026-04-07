// Tests for Problem 2589: Minimum Time to Complete All Tasks
// Java reference: src/test/java/g2501_2600/s2589_minimum_time_to_complete_all_tasks/SolutionTest.java

use leetcode_in_rust::s2589::minimum_time_to_complete_all_tasks::Solution;

#[test]
fn test_find_minimum_time() {
    assert_eq!(
        Solution::find_minimum_time(vec![vec![2, 3, 1], vec![4, 5, 1], vec![1, 5, 2]]),
        2
    );
}

#[test]
fn test_find_minimum_time2() {
    assert_eq!(
        Solution::find_minimum_time(vec![vec![1, 3, 2], vec![2, 5, 3], vec![5, 6, 2]]),
        4
    );
}
