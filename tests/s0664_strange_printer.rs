// Tests for Problem 0664: Strange Printer
// Java reference: src/test/java/g0601_0700/s0664_strange_printer/SolutionTest.java

use leetcode_in_rust::s0664::strange_printer::Solution;

#[test]
fn test_strange_printer() {
    assert_eq!(Solution::strange_printer("aaabbb".to_string()), 2);
}

#[test]
fn test_strange_printer2() {
    assert_eq!(Solution::strange_printer("aba".to_string()), 2);
}
