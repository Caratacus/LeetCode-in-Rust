// Tests for Problem 0406: Queue Reconstruction by Height
// Java reference: src/test/java/g0401_0500/s0406_queue_reconstruction_by_height/SolutionTest.java

use leetcode_in_rust::s0406::queue_reconstruction_by_height::Solution;

#[test]
fn test_reconstruct_queue() {
    let people = vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1], vec![5, 2]];
    let expected = vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7, 1]];
    assert_eq!(Solution::reconstruct_queue(people), expected);
}

#[test]
fn test_reconstruct_queue2() {
    let people = vec![vec![6, 0], vec![5, 0], vec![4, 0], vec![3, 2], vec![2, 2], vec![1, 4]];
    let expected = vec![vec![4, 0], vec![5, 0], vec![2, 2], vec![3, 2], vec![1, 4], vec![6, 0]];
    assert_eq!(Solution::reconstruct_queue(people), expected);
}
