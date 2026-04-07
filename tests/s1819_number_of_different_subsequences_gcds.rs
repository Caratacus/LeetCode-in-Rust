// Tests for Problem 1819: Number of Different Subsequences GCDs
// Java reference: src/test/java/g1801_1900/s1819_number_of_different_subsequences_gcds/SolutionTest.java

use leetcode_in_rust::s1819::number_of_different_subsequences_gcds::Solution;

#[test]
fn test_count_different_subsequence_gc_ds() {
    assert_eq!(Solution::count_different_subsequence_gc_ds(vec![6, 10, 3]), 5);
}

#[test]
fn test_count_different_subsequence_gc_ds2() {
    assert_eq!(
        Solution::count_different_subsequence_gc_ds(vec![5, 15, 40, 5, 6]),
        7
    );
}
