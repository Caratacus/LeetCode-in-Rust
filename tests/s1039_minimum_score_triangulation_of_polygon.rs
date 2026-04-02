// Tests for Problem 1039: Minimum Score Triangulation of Polygon
// Java reference: src/test/java/g1001_1100/s1039_minimum_score_triangulation_of_polygon/SolutionTest.java

use leetcode_in_rust::s1039::minimum_score_triangulation_of_polygon::Solution;

#[test]
fn test_min_score_triangulation() {
    assert_eq!(Solution::min_score_triangulation(vec![1, 2, 3]), 6);
}

#[test]
fn test_min_score_triangulation2() {
    assert_eq!(Solution::min_score_triangulation(vec![3, 7, 4, 5]), 144);
}

#[test]
fn test_min_score_triangulation3() {
    assert_eq!(Solution::min_score_triangulation(vec![1, 3, 1, 4, 1, 5]), 13);
}
