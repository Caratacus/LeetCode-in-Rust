// Tests for Problem 0768: Max Chunks To Make Sorted II
// Java reference: src/test/java/g0701_0800/s0768_max_chunks_to_make_sorted_ii/SolutionTest.java

use leetcode_in_rust::s0768::max_chunks_to_make_sorted_ii::Solution;

#[test]
fn test_max_chunks_to_sorted() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
}

#[test]
fn test_max_chunks_to_sorted2() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
}
