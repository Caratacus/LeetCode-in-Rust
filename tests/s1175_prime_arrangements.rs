// Tests for Problem 1175: Prime Arrangements
// Java reference: src/test/java/g1101_1200/s1175_prime_arrangements/SolutionTest.java

use leetcode_in_rust::s1175::prime_arrangements::Solution;

#[test]
fn test_num_prime_arrangements() {
    assert_eq!(Solution::num_prime_arrangements(5), 12);
}

#[test]
fn test_num_prime_arrangements2() {
    assert_eq!(Solution::num_prime_arrangements(100), 682289015);
}
