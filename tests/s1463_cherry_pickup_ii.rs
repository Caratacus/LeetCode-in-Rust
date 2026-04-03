// Tests for Problem 1463: Cherry Pickup II
// Java reference: src/test/java/g1401_1500/s1463_cherry_pickup_ii/SolutionTest.java

use leetcode_in_rust::s1463::cherry_pickup_ii::Solution;

#[test]
fn test_cherry_pickup() {
    let grid = vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]];
    assert_eq!(Solution::cherry_pickup(grid), 24);
}

#[test]
fn test_cherry_pickup2() {
    let grid = vec![
        vec![1, 0, 0, 0, 0, 0, 1],
        vec![2, 0, 0, 0, 0, 3, 0],
        vec![2, 0, 9, 0, 0, 0, 0],
        vec![0, 3, 0, 5, 4, 0, 0],
        vec![1, 0, 2, 3, 0, 0, 6],
    ];
    assert_eq!(Solution::cherry_pickup(grid), 28);
}
