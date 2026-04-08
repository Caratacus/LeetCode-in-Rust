// Tests for Problem 3082: Find the Sum of the Power of All Subsequences
// Java reference: src/test/java/g3001_3100/s3082_find_the_sum_of_the_power_of_all_subsequences/SolutionTest.java

use leetcode_in_rust::s3082::find_the_sum_of_the_power_of_all_subsequences::Solution;

#[test]
fn test_sum_of_power() {
    assert_eq!(Solution::sum_of_power(vec![2, 3, 3], 5), 4);
}

#[test]
fn test_sum_of_power2() {
    assert_eq!(Solution::sum_of_power(vec![1, 2, 3], 7), 0);
}
