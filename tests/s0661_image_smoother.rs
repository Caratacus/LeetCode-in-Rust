// Tests for Problem 0661: Image Smoother
// Java reference: src/test/java/g0601_0700/s0661_image_smoother/SolutionTest.java

use leetcode_in_rust::s0661::image_smoother::Solution;

#[test]
fn test_image_smoother() {
    assert_eq!(
        Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );
}

#[test]
fn test_image_smoother2() {
    assert_eq!(
        Solution::image_smoother(vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]]),
        vec![vec![137, 141, 137], vec![141, 138, 141], vec![137, 141, 137]]
    );
}
