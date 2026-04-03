// Tests for Problem 1713: Minimum Operations to Make a Subsequence
// Java reference: src/test/java/g1701_1800/s1713_minimum_operations_to_make_a_subsequence/SolutionTest.java

use leetcode_in_rust::s1713::minimum_operations_to_make_a_subsequence::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(
        Solution::min_operations(vec![5, 1, 3], vec![9, 4, 2, 3, 4]),
        2
    );
}

#[test]
fn test_min_operations2() {
    assert_eq!(
        Solution::min_operations(vec![6, 4, 8, 1, 3, 2], vec![4, 7, 6, 2, 3, 8, 6, 1]),
        3
    );
}
