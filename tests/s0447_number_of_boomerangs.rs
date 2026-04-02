// Tests for Problem 0447: Number of Boomerangs
// Java reference: src/test/java/g0401_0500/s0447_number_of_boomerangs/SolutionTest.java

use leetcode_in_rust::s0447::number_of_boomerangs::Solution;

#[test]
fn test_number_of_boomerangs() {
    assert_eq!(Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]), 2);
}

#[test]
fn test_number_of_boomerangs2() {
    assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 2);
}

#[test]
fn test_number_of_boomerangs3() {
    assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1]]), 0);
}
