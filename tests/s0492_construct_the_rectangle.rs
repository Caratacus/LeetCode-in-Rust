// Tests for Problem 0492: Construct the Rectangle
// Java reference: src/test/java/g0401_0500/s0492_construct_the_rectangle/SolutionTest.java

use leetcode_in_rust::s0492::construct_the_rectangle::Solution;

#[test]
fn test_construct_rectangle() {
    assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
}

#[test]
fn test_construct_rectangle2() {
    assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
}

#[test]
fn test_construct_rectangle3() {
    assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
}
