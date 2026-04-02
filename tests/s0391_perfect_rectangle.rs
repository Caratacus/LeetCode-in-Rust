// Tests for Problem 0391: Perfect Rectangle
// Java reference: src/test/java/g0301_0400/s0391_perfect_rectangle/SolutionTest.java

use leetcode_in_rust::s0391::perfect_rectangle::Solution;

#[test]
fn test_is_rectangle_cover() {
    // 5 rectangles that form a perfect rectangle
    let rectangles = vec![
        vec![1, 1, 3, 3],
        vec![3, 1, 4, 2],
        vec![3, 2, 4, 4],
        vec![1, 3, 2, 4],
        vec![2, 3, 3, 4],
    ];
    assert_eq!(Solution::is_rectangle_cover(rectangles), true);
}

#[test]
fn test_is_rectangle_cover2() {
    // 2 separate rectangles, not forming a perfect rectangle
    let rectangles = vec![
        vec![1, 1, 2, 3],
        vec![1, 3, 2, 4],
        vec![3, 1, 4, 2],
        vec![3, 2, 4, 4],
    ];
    assert_eq!(Solution::is_rectangle_cover(rectangles), false);
}

#[test]
fn test_is_rectangle_cover3() {
    // 2 overlapping rectangles
    let rectangles = vec![
        vec![1, 1, 3, 3],
        vec![3, 1, 4, 2],
        vec![1, 3, 2, 4],
        vec![2, 2, 4, 4],
    ];
    assert_eq!(Solution::is_rectangle_cover(rectangles), false);
}

#[test]
fn test_is_rectangle_cover4() {
    // 5 rectangles with gap
    let rectangles = vec![
        vec![1, 1, 3, 3],
        vec![3, 1, 4, 2],
        vec![1, 3, 2, 4],
        vec![3, 2, 4, 4],
        vec![2, 3, 3, 4],
    ];
    assert_eq!(Solution::is_rectangle_cover(rectangles), false);
}
