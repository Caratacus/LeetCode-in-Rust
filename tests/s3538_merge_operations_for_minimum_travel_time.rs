// Tests for Problem 3538: Merge Operations for Minimum Travel Time
// Java reference: src/test/java/g3501_3600/s3538_merge_operations_for_minimum_travel_time/SolutionTest.java

use leetcode_in_rust::s3538::merge_operations_for_minimum_travel_time::Solution;

#[test]
fn test_min_travel_time() {
    assert_eq!(Solution::min_travel_time(10, 4, 1, vec![0, 3, 8, 10], vec![5, 8, 3, 6]), 62);
}

#[test]
fn test_min_travel_time2() {
    assert_eq!(Solution::min_travel_time(5, 5, 1, vec![0, 1, 2, 3, 5], vec![8, 3, 9, 3, 3]), 34);
}
