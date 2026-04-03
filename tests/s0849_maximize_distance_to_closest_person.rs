// Tests for Problem 0849: Maximize Distance to Closest Person
// Java reference: src/test/java/g0801_0900/s0849_maximize_distance_to_closest_person/SolutionTest.java

use leetcode_in_rust::s0849::maximize_distance_to_closest_person::Solution;

#[test]
fn test_max_dist_to_closest() {
    assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
}

#[test]
fn test_max_dist_to_closest2() {
    assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0]), 3);
}

#[test]
fn test_max_dist_to_closest3() {
    assert_eq!(Solution::max_dist_to_closest(vec![0, 1]), 1);
}
