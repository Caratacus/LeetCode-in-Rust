// Tests for Problem 1591: Strange Printer II
// Java reference: src/test/java/g1501_1600/s1591_strange_printer_ii/SolutionTest.java

use leetcode_in_rust::s1591::strange_printer_ii::Solution;

#[test]
fn test_is_printable() {
    assert!(Solution::is_printable(vec![
        vec![1, 1, 1, 1],
        vec![1, 2, 2, 1],
        vec![1, 2, 2, 1],
        vec![1, 1, 1, 1]
    ]));
}

#[test]
fn test_is_printable2() {
    assert!(Solution::is_printable(vec![
        vec![1, 1, 1, 1],
        vec![1, 1, 3, 3],
        vec![1, 1, 3, 4],
        vec![5, 5, 1, 4]
    ]));
}

#[test]
fn test_is_printable3() {
    assert!(!Solution::is_printable(vec![
        vec![1, 2, 1],
        vec![2, 1, 2],
        vec![1, 2, 1]
    ]));
}
