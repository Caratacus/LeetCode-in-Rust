// Tests for Problem 2162: Minimum Cost to Set Cooking Time
// Java reference: src/test/java/g2101_2200/s2162_minimum_cost_to_set_cooking_time/SolutionTest.java

use leetcode_in_rust::s2162::minimum_cost_to_set_cooking_time::Solution;

#[test]
fn test_min_cost_set_time() {
    assert_eq!(Solution::min_cost_set_time(1, 2, 1, 600), 6);
}

#[test]
fn test_min_cost_set_time2() {
    assert_eq!(Solution::min_cost_set_time(0, 1, 2, 76), 6);
}

#[test]
fn test_min_cost_set_time3() {
    assert_eq!(Solution::min_cost_set_time(0, 9, 18, 460), 81);
}
