// Tests for Problem 2614: Prime in Diagonal
// Java reference: src/test/java/g2601_2700/s2614_prime_in_diagonal/SolutionTest.java

use leetcode_in_rust::s2614::prime_in_diagonal::Solution;

#[test]
fn test_diagonal_prime() {
    assert_eq!(
        Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]]),
        11
    );
}

#[test]
fn test_diagonal_prime2() {
    assert_eq!(
        Solution::diagonal_prime(vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]]),
        17
    );
}
