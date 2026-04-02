// Tests for Problem 1162: As Far from Land as Possible
// Java reference: src/test/java/g1101_1200/s1162_as_far_from_land_as_possible/SolutionTest.java

use leetcode_in_rust::s1162::as_far_from_land_as_possible::Solution;

#[test]
fn test_max_distance() {
    assert_eq!(
        Solution::max_distance(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]),
        2
    );
}

#[test]
fn test_max_distance2() {
    assert_eq!(
        Solution::max_distance(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
        4
    );
}
