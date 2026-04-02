// Tests for Problem 0840: Magic Squares In Grid
// Java reference: src/test/java/g0801_0900/s0840_magic_squares_in_grid/SolutionTest.java

use leetcode_in_rust::s0840::magic_squares_in_grid::Solution;

#[test]
fn test_num_magic_squares_inside() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![
            vec![4, 3, 8, 4],
            vec![9, 5, 1, 9],
            vec![2, 7, 6, 2]
        ]),
        1
    );
}

#[test]
fn test_num_magic_squares_inside2() {
    assert_eq!(Solution::num_magic_squares_inside(vec![vec![8]]), 0);
}

#[test]
fn test_num_magic_squares_inside3() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![vec![1, 2], vec![3, 4]]),
        0
    );
}

#[test]
fn test_num_magic_squares_inside4() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![
            vec![4, 3, 8, 4, 3],
            vec![9, 5, 1, 9, 5],
            vec![2, 7, 6, 2, 7],
            vec![4, 3, 8, 4, 3],
            vec![9, 5, 1, 9, 5]
        ]),
        1
    );
}

#[test]
fn test_num_magic_squares_inside5() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![vec![10, 3, 8], vec![9, 5, 1], vec![2, 7, 6]]),
        0
    );
}

#[test]
fn test_num_magic_squares_inside6() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![
            vec![4, 3, 8, 4],
            vec![9, 5, 1, 9],
            vec![2, 7, 6, 2],
            vec![4, 3, 8, 4]
        ]),
        1
    );
}

#[test]
fn test_num_magic_squares_inside7() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![
            vec![2, 2, 2],
            vec![2, 2, 2],
            vec![2, 2, 2]
        ]),
        0
    );
}

#[test]
fn test_num_magic_squares_inside8() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]]),
        1
    );
}

#[test]
fn test_num_magic_squares_inside9() {
    assert_eq!(
        Solution::num_magic_squares_inside(vec![
            vec![8, 1, 6, 8, 1, 6],
            vec![3, 5, 7, 3, 5, 7],
            vec![4, 9, 2, 4, 9, 2]
        ]),
        2
    );
}
