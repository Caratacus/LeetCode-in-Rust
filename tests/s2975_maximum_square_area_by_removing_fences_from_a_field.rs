// Tests for Problem 2975: Maximum Square Area by Removing Fences From a Field
// Java reference: src/test/java/g2901_3000/s2975_maximum_square_area_by_removing_fences_from_a_field/SolutionTest.java

use leetcode_in_rust::s2975::maximum_square_area_by_removing_fences_from_a_field::Solution;

#[test]
fn test_maximize_square_area() {
    assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
}

#[test]
fn test_maximize_square_area2() {
    assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
}
