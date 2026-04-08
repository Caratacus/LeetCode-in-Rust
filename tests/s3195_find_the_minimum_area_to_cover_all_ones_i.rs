// Tests for Problem 3195: Find the Minimum Area to Cover All Ones I
// Java reference: src/test/java/g3101_3200/s3195_find_the_minimum_area_to_cover_all_ones_i/SolutionTest.java

use leetcode_in_rust::s3195::find_the_minimum_area_to_cover_all_ones_i::Solution;

#[test]
fn test_minimum_area() {
    assert_eq!(Solution::minimum_area(vec![vec![0, 1, 0], vec![1, 0, 1]]), 6);
}

#[test]
fn test_minimum_area2() {
    assert_eq!(Solution::minimum_area(vec![vec![1, 0], vec![0, 0]]), 1);
}
