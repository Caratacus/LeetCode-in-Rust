// Tests for Problem 3454: Separate Squares II
// Java reference: src/test/java/g3401_3500/s3454_separate_squares_ii/SolutionTest.java

use leetcode_in_rust::s3454::separate_squares_ii::Solution;

#[test]
fn test_separate_squares() {
    let result = Solution::separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]);
    assert!((result - 1.0).abs() < 1e-5);
}

#[test]
fn test_separate_squares2() {
    let result = Solution::separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]);
    assert!((result - 1.0).abs() < 1e-5);
}
