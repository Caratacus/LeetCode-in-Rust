// Tests for Problem 2848: Points That Intersect With Cars
// Java reference: src/test/java/g2801_2900/s2848_points_that_intersect_with_cars/SolutionTest.java

use leetcode_in_rust::s2848::points_that_intersect_with_cars::Solution;

#[test]
fn test_number_of_points() {
    assert_eq!(
        Solution::number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]]),
        7
    );
}

#[test]
fn test_number_of_points2() {
    assert_eq!(
        Solution::number_of_points(vec![vec![1, 3], vec![5, 8]]),
        7
    );
}
