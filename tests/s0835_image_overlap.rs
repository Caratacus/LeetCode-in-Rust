// Tests for Problem 0835: Image Overlap
// Java reference: src/test/java/g0801_0900/s0835_image_overlap/SolutionTest.java

use leetcode_in_rust::s0835::image_overlap::Solution;

#[test]
fn test_largest_overlap() {
    assert_eq!(
        Solution::largest_overlap(
            vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
        ),
        3
    );
}

#[test]
fn test_largest_overlap2() {
    assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![1]]), 1);
}

#[test]
fn test_largest_overlap3() {
    assert_eq!(Solution::largest_overlap(vec![vec![0]], vec![vec![0]]), 0);
}
