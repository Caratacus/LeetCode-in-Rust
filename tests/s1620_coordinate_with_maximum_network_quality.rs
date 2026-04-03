// Tests for Problem 1620: Coordinate With Maximum Network Quality
// Java reference: src/test/java/g1601_1700/s1620_coordinate_with_maximum_network_quality/SolutionTest.java

use leetcode_in_rust::s1620::coordinate_with_maximum_network_quality::Solution;

#[test]
fn test_best_coordinate() {
    assert_eq!(
        Solution::best_coordinate(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2),
        vec![2, 1]
    );
}

#[test]
fn test_best_coordinate2() {
    assert_eq!(Solution::best_coordinate(vec![vec![23, 11, 21]], 9), vec![23, 11]);
}

#[test]
fn test_best_coordinate3() {
    assert_eq!(
        Solution::best_coordinate(vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]], 2),
        vec![1, 2]
    );
}
