// Tests for Problem 1652: Defuse the Bomb
// Java reference: src/test/java/g1601_1700/s1652_defuse_the_bomb/SolutionTest.java

use leetcode_in_rust::s1652::defuse_the_bomb::Solution;

#[test]
fn test_decrypt() {
    assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
}

#[test]
fn test_decrypt2() {
    assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
}

#[test]
fn test_decrypt3() {
    assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
}
