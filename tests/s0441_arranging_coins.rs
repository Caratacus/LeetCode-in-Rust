// Tests for Problem 0441: Arranging Coins
// Java reference: src/test/java/g0401_0500/s0441_arranging_coins/SolutionTest.java

use leetcode_in_rust::s0441::arranging_coins::Solution;

#[test]
fn test_arrange_coins() {
    assert_eq!(Solution::arrange_coins(5), 2);
}

#[test]
fn test_arrange_coins2() {
    assert_eq!(Solution::arrange_coins(8), 3);
}

#[test]
fn test_arrange_coins3() {
    assert_eq!(Solution::arrange_coins(0), 0);
}

#[test]
fn test_arrange_coins4() {
    assert_eq!(Solution::arrange_coins(1), 1);
}

#[test]
fn test_arrange_coins5() {
    assert_eq!(Solution::arrange_coins(2), 1);
}

#[test]
fn test_arrange_coins6() {
    assert_eq!(Solution::arrange_coins(3), 2);
}

#[test]
fn test_arrange_coins7() {
    assert_eq!(Solution::arrange_coins(6), 3);
}

#[test]
fn test_arrange_coins8() {
    assert_eq!(Solution::arrange_coins(2147483647), 65535);
}

#[test]
fn test_arrange_coins9() {
    assert_eq!(Solution::arrange_coins(7), 3);
}
