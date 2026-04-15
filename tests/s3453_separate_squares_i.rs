// Tests for Problem 3453: Separate Squares I
// Java reference: src/test/java/g3401_3500/s3453_separate_squares_i/SolutionTest.java

use leetcode_in_rust::s3453::separate_squares_i::Solution;

#[test]
fn test_separate_squares() {
    let result = Solution::separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]);
    assert!((result - 1.0).abs() < 1e-5);
}

#[test]
fn test_separate_squares2() {
    let result = Solution::separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]);
    assert!((result - 1.1666666666666667).abs() < 1e-5);
}
