// Tests for Problem 1377: Frog Position After T Seconds
// Java reference: src/test/java/g1301_1400/s1377_frog_position_after_t_seconds/SolutionTest.java

use leetcode_in_rust::s1377::frog_position_after_t_seconds::Solution;

#[test]
fn test_frog_position() {
    let result = Solution::frog_position(
        7,
        vec![vec![1, 2], vec![1, 3], vec![1, 7], vec![2, 4], vec![2, 6], vec![3, 5]],
        2,
        4,
    );
    assert!((result - 0.16666666666666666).abs() < 1e-9);
}

#[test]
fn test_frog_position2() {
    let result = Solution::frog_position(
        7,
        vec![vec![1, 2], vec![1, 3], vec![1, 7], vec![2, 4], vec![2, 6], vec![3, 5]],
        1,
        7,
    );
    assert!((result - 0.3333333333333333).abs() < 1e-9);
}
