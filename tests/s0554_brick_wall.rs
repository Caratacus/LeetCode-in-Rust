// Tests for Problem 0554: Brick Wall
// Java reference: src/test/java/g0501_0600/s0554_brick_wall/SolutionTest.java

use leetcode_in_rust::s0554::brick_wall::Solution;

#[test]
fn test_least_bricks() {
    assert_eq!(
        Solution::least_bricks(vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1]
        ]),
        2
    );
}

#[test]
fn test_least_bricks2() {
    assert_eq!(
        Solution::least_bricks(vec![vec![1], vec![1], vec![1]]),
        3
    );
}
