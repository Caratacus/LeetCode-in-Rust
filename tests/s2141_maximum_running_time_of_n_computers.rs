// Tests for Problem 2141: Maximum Running Time of N Computers
// Java reference: src/test/java/g2101_2200/s2141_maximum_running_time_of_n_computers/SolutionTest.java

use leetcode_in_rust::s2141::maximum_running_time_of_n_computers::Solution;

#[test]
fn test_max_run_time() {
    assert_eq!(Solution::max_run_time(2, vec![3, 3, 3]), 4);
}

#[test]
fn test_max_run_time2() {
    assert_eq!(Solution::max_run_time(2, vec![1, 1, 1, 1]), 2);
}
