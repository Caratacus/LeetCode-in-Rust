// Tests for Problem 3392: Count Subarrays of Length Three With a Condition
// Java reference: src/test/java/g3301_3400/s3392_count_subarrays_of_length_three_with_a_condition/SolutionTest.java

use leetcode_in_rust::s3392::count_subarrays_of_length_three_with_a_condition::Solution;

#[test]
fn test_count_subarrays() {
    assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 4, 1]), 1);
}

#[test]
fn test_count_subarrays2() {
    assert_eq!(Solution::count_subarrays(vec![1, 1, 1]), 0);
}
