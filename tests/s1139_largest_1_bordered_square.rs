// Tests for Problem 1139: Largest 1-Bordered Square
// Java reference: src/test/java/g1101_1200/s1139_largest_1_bordered_square/SolutionTest.java

use leetcode_in_rust::s1139::largest_1_bordered_square::Solution;

#[test]
fn test_largest1_bordered_square() {
    assert_eq!(
        Solution::largest1_bordered_square(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        9
    );
}

#[test]
fn test_largest1_bordered_square2() {
    assert_eq!(Solution::largest1_bordered_square(vec![vec![1, 1, 0, 0]]), 1);
}
