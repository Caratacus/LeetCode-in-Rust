// Tests for Problem 3312: Sorted GCD Pair Queries
// Java reference: src/test/java/g3301_3400/s3312_sorted_gcd_pair_queries/SolutionTest.java

use leetcode_in_rust::s3312::sorted_gcd_pair_queries::Solution;

#[test]
fn test_gcd_values() {
    assert_eq!(
        Solution::gcd_values(vec![2, 3, 4], vec![0, 2, 2]),
        vec![1, 2, 2]
    );
}

#[test]
fn test_gcd_values2() {
    assert_eq!(
        Solution::gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0]),
        vec![4, 2, 1, 1]
    );
}
