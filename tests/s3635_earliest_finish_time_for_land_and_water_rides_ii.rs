// Tests for Problem 3635: Earliest Finish Time for Land and Water Rides II
// Java reference: src/test/java/g3601_3700/s3635_earliest_finish_time_for_land_and_water_rides_ii/SolutionTest.java
use leetcode_in_rust::s3635::earliest_finish_time_for_land_and_water_rides_ii::Solution;
#[test]
fn test_earliest_finish_time() {
    assert_eq!(Solution::earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]), 9);
}
#[test]
fn test_earliest_finish_time2() {
    assert_eq!(Solution::earliest_finish_time(vec![5], vec![3], vec![1], vec![10]), 14);
}
