// Tests for Problem 1840: Maximum Building Height
// Java reference: src/test/java/g1801_1900/s1840_maximum_building_height/SolutionTest.java

use leetcode_in_rust::s1840::maximum_building_height::Solution;

#[test]
fn test_max_building() {
    assert_eq!(Solution::max_building(5, vec![vec![2, 1], vec![4, 1]]), 2);
}

#[test]
fn test_max_building2() {
    assert_eq!(Solution::max_building(6, vec![]), 5);
}

#[test]
fn test_max_building3() {
    assert_eq!(
        Solution::max_building(10, vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]]),
        5
    );
}
