// Tests for Problem 0753: Cracking the Safe
// Java reference: src/test/java/g0701_0800/s0753_cracking_the_safe/SolutionTest.java

use leetcode_in_rust::s0753::cracking_the_safe::Solution;

#[test]
fn test_crack_safe() {
    assert_eq!(Solution::crack_safe(1, 2), "01");
}

#[test]
fn test_crack_safe2() {
    assert_eq!(Solution::crack_safe(2, 2), "00110");
}
