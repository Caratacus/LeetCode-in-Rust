// Tests for Problem 1201: Ugly Number III
// Java reference: src/test/java/g1201_1300/s1201_ugly_number_iii/SolutionTest.java

use leetcode_in_rust::s1201::ugly_number_iii::Solution;

#[test]
fn test_nth_ugly_number() {
    assert_eq!(Solution::nth_ugly_number(3, 2, 3, 5), 4);
}

#[test]
fn test_nth_ugly_number2() {
    assert_eq!(Solution::nth_ugly_number(4, 2, 3, 4), 6);
}

#[test]
fn test_nth_ugly_number3() {
    assert_eq!(Solution::nth_ugly_number(5, 2, 11, 13), 10);
}
