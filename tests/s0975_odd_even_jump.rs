// Tests for Problem 0975: Odd Even Jump
// Java reference: src/test/java/g0901_1000/s0975_odd_even_jump/SolutionTest.java

use leetcode_in_rust::s0975::odd_even_jump::Solution;

#[test]
fn test_odd_even_jumps() {
    let result = Solution::odd_even_jumps(vec![10, 13, 12, 14, 15]);
    assert_eq!(result, 2);
}

#[test]
fn test_odd_even_jumps2() {
    let result = Solution::odd_even_jumps(vec![2, 3, 1, 1, 4]);
    assert_eq!(result, 3);
}

#[test]
fn test_odd_even_jumps3() {
    let result = Solution::odd_even_jumps(vec![5, 1, 3, 4, 2]);
    assert_eq!(result, 3);
}
