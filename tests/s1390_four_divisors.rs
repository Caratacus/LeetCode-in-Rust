// Tests for Problem 1390: Four Divisors
// Java reference: src/test/java/g1301_1400/s1390_four_divisors/SolutionTest.java

use leetcode_in_rust::s1390::four_divisors::Solution;

#[test]
fn test_sum_four_divisors() {
    assert_eq!(Solution::sum_four_divisors(vec![21, 4, 7]), 32);
}

#[test]
fn test_sum_four_divisors2() {
    assert_eq!(Solution::sum_four_divisors(vec![21, 21]), 64);
}

#[test]
fn test_sum_four_divisors3() {
    assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
}
