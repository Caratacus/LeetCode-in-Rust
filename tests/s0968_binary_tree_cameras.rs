// Tests for Problem 0968: Binary Tree Cameras
// Java reference: src/test/java/g0901_1000/s0968_binary_tree_cameras/SolutionTest.java

use leetcode_in_rust::s0968::binary_tree_cameras::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_min_camera_cover() {
    let root = tree_from_vec(vec![Some(0), Some(0), None, Some(0), Some(0)]);
    let result = Solution::min_camera_cover(root);
    assert_eq!(result, 1);
}

#[test]
fn test_min_camera_cover2() {
    let root = tree_from_vec(vec![Some(0), Some(0), None, Some(0), None, Some(0), None, None, Some(0)]);
    let result = Solution::min_camera_cover(root);
    assert_eq!(result, 2);
}
