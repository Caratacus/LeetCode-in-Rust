// Tests for Problem 2411: Smallest Subarrays With Maximum Bitwise OR
// Java reference: src/test/java/g2401_2500/s2411_smallest_subarrays_with_maximum_bitwise_or/SolutionTest.java

use leetcode_in_rust::s2411::smallest_subarrays_with_maximum_bitwise_or::Solution;

#[test]
fn test_smallest_subarrays() {
    assert_eq!(
        Solution::smallest_subarrays(vec![1, 0, 2, 1, 3]),
        vec![3, 3, 2, 2, 1]
    );
}

#[test]
fn test_smallest_subarrays2() {
    assert_eq!(
        Solution::smallest_subarrays(vec![1, 2]),
        vec![2, 1]
    );
}
