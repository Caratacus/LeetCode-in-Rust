// Tests for Problem 2971: Find Polygon With the Largest Perimeter
// Java reference: src/test/java/g2901_3000/s2971_find_polygon_with_the_largest_perimeter/SolutionTest.java

use leetcode_in_rust::s2971::find_polygon_with_the_largest_perimeter::Solution;

#[test]
fn test_largest_perimeter() {
    assert_eq!(Solution::largest_perimeter(vec![5, 5, 5]), 15);
}

#[test]
fn test_largest_perimeter2() {
    assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
}

#[test]
fn test_largest_perimeter3() {
    assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
}
