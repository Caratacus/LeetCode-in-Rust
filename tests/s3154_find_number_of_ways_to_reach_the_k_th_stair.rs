// Tests for Problem 3154: Find Number of Ways to Reach the K-th Stair
// Java reference: src/test/java/g3101_3200/s3154_find_number_of_ways_to_reach_the_k_th_stair/SolutionTest.java

use leetcode_in_rust::s3154::find_number_of_ways_to_reach_the_k_th_stair::Solution;
#[test]
fn test_ways_to_reach_stair() {
    assert_eq!(Solution::ways_to_reach_stair(0), 2);
}
#[test]
fn test_ways_to_reach_stair2() {
    assert_eq!(Solution::ways_to_reach_stair(1), 4);
}
