// Tests for Problem 2481: Minimum Cuts to Divide a Circle
// Java reference: src/test/java/g2401_2500/s2481_minimum_cuts_to_divide_a_circle/SolutionTest.java

use leetcode_in_rust::s2481::minimum_cuts_to_divide_a_circle::Solution;

#[test]
fn test_number_of_cuts() {
    assert_eq!(Solution::number_of_cuts(4), 2);
}

#[test]
fn test_number_of_cuts2() {
    assert_eq!(Solution::number_of_cuts(3), 3);
}

#[test]
fn test_number_of_cuts3() {
    assert_eq!(Solution::number_of_cuts(1), 0);
}

#[test]
fn test_number_of_cuts4() {
    assert_eq!(Solution::number_of_cuts(6), 3);
}

#[test]
fn test_number_of_cuts5() {
    assert_eq!(Solution::number_of_cuts(5), 5);
}

#[test]
fn test_number_of_cuts6() {
    assert_eq!(Solution::number_of_cuts(100), 50);
}

#[test]
fn test_number_of_cuts7() {
    assert_eq!(Solution::number_of_cuts(101), 101);
}

#[test]
fn test_number_of_cuts8() {
    assert_eq!(Solution::number_of_cuts(2), 1);
}

#[test]
fn test_number_of_cuts9() {
    assert_eq!(Solution::number_of_cuts(3), 3);
}
