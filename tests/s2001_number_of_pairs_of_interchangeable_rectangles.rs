// Tests for Problem 2001: Number of Pairs of Interchangeable Rectangles
// Java reference: src/test/java/g2001_2100/s2001_number_of_pairs_of_interchangeable_rectangles/SolutionTest.java

use leetcode_in_rust::s2001::number_of_pairs_of_interchangeable_rectangles::Solution;

#[test]
fn test_interchangeable_rectangles() {
    assert_eq!(
        Solution::interchangeable_rectangles(vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]]),
        6
    );
}

#[test]
fn test_interchangeable_rectangles2() {
    assert_eq!(
        Solution::interchangeable_rectangles(vec![vec![4, 5], vec![7, 8]]),
        0
    );
}
