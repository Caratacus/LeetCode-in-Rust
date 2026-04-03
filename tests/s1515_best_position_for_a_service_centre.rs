// Tests for Problem 1515: Best Position for a Service Centre
// Java reference: src/test/java/g1501_1600/s1515_best_position_for_a_service_centre/SolutionTest.java

use leetcode_in_rust::s1515::best_position_for_a_service_centre::Solution;

#[test]
fn test_get_min_dist_sum() {
    let result = Solution::get_min_dist_sum(vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]]);
    assert!((result - 4.00).abs() < 1e-5);
}

#[test]
fn test_get_min_dist_sum2() {
    let result = Solution::get_min_dist_sum(vec![vec![1, 1], vec![3, 3]]);
    assert!((result - 2.82842712474619).abs() < 1e-5);
}
