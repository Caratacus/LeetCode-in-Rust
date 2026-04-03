// Tests for Problem 1642: Furthest Building You Can Reach
// Java reference: src/test/java/g1601_1700/s1642_furthest_building_you_can_reach/SolutionTest.java

use leetcode_in_rust::s1642::furthest_building_you_can_reach::Solution;

#[test]
fn test_furthest_building() {
    assert_eq!(Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1), 4);
}

#[test]
fn test_furthest_building2() {
    assert_eq!(Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2), 7);
}

#[test]
fn test_furthest_building3() {
    assert_eq!(Solution::furthest_building(vec![14, 3, 19, 3], 17, 0), 3);
}
