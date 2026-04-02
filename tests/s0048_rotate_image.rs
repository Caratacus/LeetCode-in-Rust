// Tests for Problem 0048: Rotate Image
// Java reference: src/test/java/g0001_0100/s0048_rotate_image/SolutionTest.java

use leetcode_in_rust::s0048::rotate_image::Solution;

#[test]
fn test_rotate() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
}

#[test]
fn test_rotate2() {
    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix);
    assert_eq!(
        matrix,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ]
    );
}
