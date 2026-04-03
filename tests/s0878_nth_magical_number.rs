// Tests for Problem 0878: Nth Magical Number
// Java reference: src/test/java/g0801_0900/s0878_nth_magical_number/SolutionTest.java

use leetcode_in_rust::s0878::nth_magical_number::Solution;

#[test]
fn test_nth_magical_number() {
    assert_eq!(Solution::nth_magical_number(1, 2, 3), 2);
}

#[test]
fn test_nth_magical_number2() {
    assert_eq!(Solution::nth_magical_number(4, 2, 3), 6);
}
