// Tests for Problem 1137: N-th Tribonacci Number
// Java reference: src/test/java/g1101_1200/s1137_n_th_tribonacci_number/SolutionTest.java

use leetcode_in_rust::s1137::n_th_tribonacci_number::Solution;

#[test]
fn test_tribonacci() {
    assert_eq!(Solution::tribonacci(4), 4);
}

#[test]
fn test_tribonacci2() {
    assert_eq!(Solution::tribonacci(25), 1389537);
}
