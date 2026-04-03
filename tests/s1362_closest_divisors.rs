// Tests for Problem 1362: Closest Divisors
// Java reference: src/test/java/g1301_1400/s1362_closest_divisors/SolutionTest.java

use leetcode_in_rust::s1362::closest_divisors::Solution;

#[test]
fn test_closest_divisors() {
    let result = Solution::closest_divisors(8);
    assert_eq!(result[0] * result[1], 9);
}

#[test]
fn test_closest_divisors2() {
    let result = Solution::closest_divisors(123);
    assert_eq!(result[0] * result[1], 124);
}
