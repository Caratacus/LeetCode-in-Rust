// Tests for Problem 1808: Maximize Number of Nice Divisors
// Java reference: src/test/java/g1801_1900/s1808_maximize_number_of_nice_divisors/SolutionTest.java

use leetcode_in_rust::s1808::maximize_number_of_nice_divisors::Solution;

#[test]
fn test_max_nice_divisors() {
    assert_eq!(Solution::max_nice_divisors(5), 6);
}

#[test]
fn test_max_nice_divisors2() {
    assert_eq!(Solution::max_nice_divisors(8), 18);
}
