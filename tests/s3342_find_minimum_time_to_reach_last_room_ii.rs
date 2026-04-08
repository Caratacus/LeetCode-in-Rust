// Tests for Problem 3342: Find Minimum Time to Reach Last Room II
// Java reference: src/test/java/g3301_3400/s3342_find_minimum_time_to_reach_last_room_ii/SolutionTest.java

use leetcode_in_rust::s3342::find_minimum_time_to_reach_last_room_ii::Solution;

#[test]
fn test_min_time_to_reach() {
    assert_eq!(Solution::min_time_to_reach(vec![vec![0, 4], vec![4, 4]]), 7);
}

#[test]
fn test_min_time_to_reach2() {
    assert_eq!(Solution::min_time_to_reach(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]]), 6);
}

#[test]
fn test_min_time_to_reach3() {
    assert_eq!(Solution::min_time_to_reach(vec![vec![0, 1], vec![1, 2]]), 4);
}
