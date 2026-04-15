// Tests for Problem 3464: Maximize the Distance Between Points on a Square
// Java reference: src/test/java/g3401_3500/s3464_maximize_the_distance_between_points_on_a_square/SolutionTest.java

use leetcode_in_rust::s3464::maximize_the_distance_between_points_on_a_square::Solution;

#[test]
fn test_max_distance() {
    assert_eq!(Solution::max_distance(2, vec![vec![0, 2], vec![2, 0], vec![2, 2], vec![0, 0]], 4), 2);
}

#[test]
fn test_max_distance2() {
    assert_eq!(Solution::max_distance(2, vec![vec![0, 0], vec![1, 2], vec![2, 0], vec![2, 2], vec![2, 1]], 4), 1);
}

#[test]
fn test_max_distance3() {
    assert_eq!(Solution::max_distance(2, vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![1, 2], vec![2, 0], vec![2, 2], vec![2, 1]], 5), 1);
}
