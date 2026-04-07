// Tests for Problem 2212: Maximum Points in an Archery Competition
// Java reference: src/test/java/g2201_2300/s2212_maximum_points_in_an_archery_competition/SolutionTest.java

use leetcode_in_rust::s2212::maximum_points_in_an_archery_competition::Solution;

#[test]
fn test_maximum_bob_points() {
    assert_eq!(
        Solution::maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]),
        vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1]
    );
}

#[test]
fn test_maximum_bob_points2() {
    assert_eq!(
        Solution::maximum_bob_points(3, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0]
    );
}
