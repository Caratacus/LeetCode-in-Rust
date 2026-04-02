// Tests for Problem 0509: Fibonacci Number
// Java reference: src/test/java/g0501_0600/s0509_fibonacci_number/SolutionTest.java

use leetcode_in_rust::s0509::fibonacci_number::Solution;

#[test]
fn test_fib() {
    assert_eq!(Solution::fib(2), 1);
}

#[test]
fn test_fib2() {
    assert_eq!(Solution::fib(3), 2);
}

#[test]
fn test_fib3() {
    assert_eq!(Solution::fib(4), 3);
}
