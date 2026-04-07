// Tests for Problem 2615: Sum of Distances
// Java reference: src/test/java/g2601_2700/s2615_sum_of_distances/SolutionTest.java

use leetcode_in_rust::s2615::sum_of_distances::Solution;

#[test]
fn test_distance() {
    assert_eq!(
        Solution::distance(vec![1, 3, 1, 1, 2]),
        vec![5i64, 0, 3, 4, 0]
    );
}

#[test]
fn test_distance2() {
    assert_eq!(Solution::distance(vec![0, 5, 3]), vec![0i64, 0, 0]);
}
