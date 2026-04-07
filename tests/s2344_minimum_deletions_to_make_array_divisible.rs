// Tests for Problem 2344: Minimum Deletions to Make Array Divisible
// Java reference: src/test/java/g2301_2400/s2344_minimum_deletions_to_make_array_divisible/SolutionTest.java

use leetcode_in_rust::s2344::minimum_deletions_to_make_array_divisible::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(
        Solution::min_operations(vec![9, 6, 9, 3, 15], vec![8, 2, 6, 10]),
        2
    );
}

#[test]
fn test_min_operations2() {
    assert_eq!(
        Solution::min_operations(vec![4, 3, 6], vec![8, 2, 6, 10]),
        -1
    );
}
