// Tests for Problem 1260: Shift 2D Grid
// Java reference: src/test/java/g1201_1300/s1260_shift_2d_grid/SolutionTest.java

use leetcode_in_rust::s1260::shift_2d_grid::Solution;

#[test]
fn test_shift_grid() {
    assert_eq!(
        Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
    );
}

#[test]
fn test_shift_grid2() {
    assert_eq!(
        Solution::shift_grid(
            vec![
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
                vec![12, 0, 21, 13]
            ],
            4
        ),
        vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10]
        ]
    );
}

#[test]
fn test_shift_grid3() {
    assert_eq!(
        Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    );
}
