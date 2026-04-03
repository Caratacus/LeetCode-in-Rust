// Tests for Problem 0719: Find K-th Smallest Pair Distance
// Java reference: src/test/java/g0701_0800/s0719_find_k_th_smallest_pair_distance/SolutionTest.java

use leetcode_in_rust::s0719::find_k_th_smallest_pair_distance::Solution;

#[test]
fn test_smallest_distance_pair() {
    assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
}

#[test]
fn test_smallest_distance_pair2() {
    assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
}

#[test]
fn test_smallest_distance_pair3() {
    assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
}
