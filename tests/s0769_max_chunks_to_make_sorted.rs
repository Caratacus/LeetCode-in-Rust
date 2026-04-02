// Tests for Problem 0769: Max Chunks To Make Sorted
// Java reference: src/test/java/g0701_0800/s0769_max_chunks_to_make_sorted/SolutionTest.java

use leetcode_in_rust::s0769::max_chunks_to_make_sorted::Solution;

#[test]
fn test_max_chunks_to_sorted() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
}

#[test]
fn test_max_chunks_to_sorted2() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
}
