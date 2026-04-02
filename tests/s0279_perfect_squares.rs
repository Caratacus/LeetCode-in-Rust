// Tests for Problem 0279: Perfect Squares
// Java reference: src/test/java/g0201_0300/s0279_perfect_squares/SolutionTest.java

use leetcode_in_rust::s0279::perfect_squares::Solution;

#[test]
fn test_num_squares() {
    assert_eq!(Solution::num_squares(12), 3);
}

#[test]
fn test_num_squares2() {
    assert_eq!(Solution::num_squares(13), 2);
}
