// Tests for Problem 2761: Prime Pairs With Target Sum
// Java reference: src/test/java/g2701_2800/s2761_prime_pairs_with_target_sum/SolutionTest.java

use leetcode_in_rust::s2761::prime_pairs_with_target_sum::Solution;

#[test]
fn test_find_prime_pairs() {
    assert_eq!(Solution::find_prime_pairs(10), vec![vec![3, 7], vec![5, 5]]);
}

#[test]
fn test_find_prime_pairs2() {
    assert_eq!(Solution::find_prime_pairs(2), vec![] as Vec<Vec<i32>>);
}
