// Tests for Problem 3000: Maximum Area of Longest Diagonal Rectangle
// Java reference: src/test/java/g2901_3000/s3000_maximum_area_of_longest_diagonal_rectangle/SolutionTest.java

use leetcode_in_rust::s3000::maximum_area_of_longest_diagonal_rectangle::Solution;

#[test]
fn test_area_of_max_diagonal() {
    assert_eq!(Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]), 48);
}

#[test]
fn test_area_of_max_diagonal2() {
    assert_eq!(Solution::area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]]), 12);
}
