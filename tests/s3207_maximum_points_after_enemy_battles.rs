// Tests for Problem 3207: Maximum Points After Enemy Battles
// Java reference: src/test/java/g3201_3300/s3207_maximum_points_after_enemy_battles/SolutionTest.java

use leetcode_in_rust::s3207::maximum_points_after_enemy_battles::Solution;

#[test]
fn test_maximum_points() {
    assert_eq!(Solution::maximum_points(vec![3, 2, 2], 2), 3);
}

#[test]
fn test_maximum_points2() {
    assert_eq!(Solution::maximum_points(vec![2], 10), 5);
}
