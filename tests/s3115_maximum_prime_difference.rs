// Tests for Problem 3115: Maximum Prime Difference
// Java reference: src/test/java/g3101_3200/s3115_maximum_prime_difference/SolutionTest.java

use leetcode_in_rust::s3115::maximum_prime_difference::Solution;

#[test]
fn test_maximum_prime_difference() {
    assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
}

#[test]
fn test_maximum_prime_difference2() {
    assert_eq!(Solution::maximum_prime_difference(vec![4, 8, 2, 8]), 0);
}
