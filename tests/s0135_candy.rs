// Tests for Problem 0135: Candy
// Java reference: src/test/java/g0121_0200/s0135_candy/SolutionTest.java

use leetcode_in_rust::s0135::candy::Solution;

#[test]
fn test_candy() {
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
}

#[test]
fn test_candy2() {
    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
}
