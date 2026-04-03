// Tests for Problem 1414: Find the Minimum Number of Fibonacci Numbers Whose Sum Is K
// Java reference: src/test/java/g1401_1500/s1414_find_the_minimum_number_of_fibonacci_numbers_whose_sum_is_k/SolutionTest.java

use leetcode_in_rust::s1414::find_the_minimum_number_of_fibonacci_numbers_whose_sum_is_k::Solution;

#[test]
fn test_find_min_fibonacci_numbers() {
    assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
}

#[test]
fn test_find_min_fibonacci_numbers2() {
    assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
}

#[test]
fn test_find_min_fibonacci_numbers3() {
    assert_eq!(Solution::find_min_fibonacci_numbers(19), 3);
}
