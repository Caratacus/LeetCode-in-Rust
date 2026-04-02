// Tests for Problem 0832: Flipping an Image
// Java reference: src/test/java/g0801_0900/s0832_flipping_an_image/SolutionTest.java

use leetcode_in_rust::s0832::flipping_an_image::Solution;

#[test]
fn test_flip_and_invert_image() {
    assert_eq!(
        Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
    );
}

#[test]
fn test_flip_and_invert_image2() {
    assert_eq!(
        Solution::flip_and_invert_image(vec![
            vec![1, 1, 0, 0],
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 0]
        ]),
        vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 1, 0]
        ]
    );
}
