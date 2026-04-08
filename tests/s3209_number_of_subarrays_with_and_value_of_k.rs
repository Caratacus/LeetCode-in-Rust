// Tests for Problem 3209: Number of Subarrays With AND Value of K
// Java reference: src/test/java/g3201_3300/s3209_number_of_subarrays_with_and_value_of_k/SolutionTest.java

use leetcode_in_rust::s3209::number_of_subarrays_with_and_value_of_k::Solution;

#[test]
fn test_count_subarrays() {
    assert_eq!(Solution::count_subarrays(vec![1, 1, 2], 1), 3);
}

#[test]
fn test_count_subarrays2() {
    assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 2), 2);
}
