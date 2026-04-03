// Tests for Problem 1723: Find Minimum Time to Finish All Jobs
// Java reference: src/test/java/g1701_1800/s1723_find_minimum_time_to_finish_all_jobs/SolutionTest.java

use leetcode_in_rust::s1723::find_minimum_time_to_finish_all_jobs::Solution;

#[test]
fn test_minimum_time_required() {
    assert_eq!(Solution::minimum_time_required(vec![3, 2, 3], 3), 3);
}

#[test]
fn test_minimum_time_required2() {
    assert_eq!(
        Solution::minimum_time_required(vec![1, 2, 4, 7, 8], 2),
        11
    );
}
