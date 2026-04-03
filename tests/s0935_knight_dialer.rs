// Tests for Problem 0935: Knight Dialer
// Java reference: src/test/java/g0901_1000/s0935_knight_dialer/SolutionTest.java

use leetcode_in_rust::s0935::knight_dialer::Solution;

#[test]
fn test_knight_dialer() {
    let result = Solution::knight_dialer(1);
    assert_eq!(result, 10);
}

#[test]
fn test_knight_dialer2() {
    let result = Solution::knight_dialer(2);
    assert_eq!(result, 20);
}

#[test]
fn test_knight_dialer3() {
    let result = Solution::knight_dialer(3131);
    assert_eq!(result, 136006598);
}
