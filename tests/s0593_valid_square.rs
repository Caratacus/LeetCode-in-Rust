// Tests for Problem 0593: Valid Square
// Java reference: src/test/java/g0501_0600/s0593_valid_square/SolutionTest.java

use leetcode_in_rust::s0593::valid_square::Solution;

#[test]
fn test_valid_square() {
    assert_eq!(
        Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
        true
    );
}

#[test]
fn test_valid_square2() {
    assert_eq!(
        Solution::valid_square(vec![1, 1], vec![5, 3], vec![3, 5], vec![7, 7]),
        false
    );
}
