// Tests for Problem 1779: Find Nearest Point That Has the Same X or Y Coordinate
// Java reference: src/test/java/g1701_1800/s1779_find_nearest_point_that_has_the_same_x_or_y_coordinate/SolutionTest.java

use leetcode_in_rust::s1779::find_nearest_point_that_has_the_same_x_or_y_coordinate::Solution;

#[test]
fn test_nearest_valid_point() {
    assert_eq!(
        Solution::nearest_valid_point(3, 4, vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]),
        2
    );
}

#[test]
fn test_nearest_valid_point2() {
    assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]), 0);
}

#[test]
fn test_nearest_valid_point3() {
    assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
}
