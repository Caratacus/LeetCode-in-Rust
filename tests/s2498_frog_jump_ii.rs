// Tests for Problem 2498: Frog Jump II
// Java reference: src/test/java/g2401_2500/s2498_frog_jump_ii/SolutionTest.java

use leetcode_in_rust::s2498::frog_jump_ii::Solution;

#[test]
fn test_max_jump() {
    assert_eq!(Solution::max_jump(vec![0, 2, 5, 6, 7]), 5);
}

#[test]
fn test_max_jump2() {
    assert_eq!(Solution::max_jump(vec![0, 3, 9]), 9);
}
