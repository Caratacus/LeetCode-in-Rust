// Tests for Problem 3483: Unique 3 Digit Even Numbers
// Java reference: src/test/java/g3401_3500/s3483_unique_3_digit_even_numbers/SolutionTest.java

use leetcode_in_rust::s3483::unique_3_digit_even_numbers::Solution;

#[test]
fn test_total_numbers() {
    assert_eq!(Solution::total_numbers(vec![1, 2, 3, 4]), 12);
}

#[test]
fn test_total_numbers2() {
    assert_eq!(Solution::total_numbers(vec![0, 2, 2]), 2);
}

#[test]
fn test_total_numbers3() {
    assert_eq!(Solution::total_numbers(vec![6, 6, 6]), 1);
}

#[test]
fn test_total_numbers4() {
    assert_eq!(Solution::total_numbers(vec![1, 3, 5]), 0);
}
