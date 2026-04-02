// Tests for Problem 0836: Rectangle Overlap
// Java reference: src/test/java/g0801_0900/s0836_rectangle_overlap/SolutionTest.java

use leetcode_in_rust::s0836::rectangle_overlap::Solution;

#[test]
fn test_is_rectangle_overlap() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]), true);
}

#[test]
fn test_is_rectangle_overlap2() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]), false);
}

#[test]
fn test_is_rectangle_overlap3() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]), false);
}

#[test]
fn test_is_rectangle_overlap4() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![0, 2, 2, 4]), false);
}

#[test]
fn test_is_rectangle_overlap5() {
    assert_eq!(Solution::is_rectangle_overlap(vec![1, 1, 3, 3], vec![1, 0, 3, 1]), false);
}

#[test]
fn test_is_rectangle_overlap6() {
    assert_eq!(
        Solution::is_rectangle_overlap(vec![-3, -3, -1, -1], vec![-2, -2, 0, 0]),
        true
    );
}

#[test]
fn test_is_rectangle_overlap7() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0, 0, 4, 4], vec![1, 1, 3, 3]), true);
}

#[test]
fn test_is_rectangle_overlap8() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![0, 0, 2, 2]), true);
}

#[test]
fn test_is_rectangle_overlap9() {
    assert_eq!(Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 1, 2, 2]), false);
}
