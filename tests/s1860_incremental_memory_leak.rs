// Tests for Problem 1860: Incremental Memory Leak
// Java reference: src/test/java/g1801_1900/s1860_incremental_memory_leak/SolutionTest.java

use leetcode_in_rust::s1860::incremental_memory_leak::Solution;

#[test]
fn test_mem_leak() {
    assert_eq!(Solution::mem_leak(2, 2), vec![3, 1, 0]);
}

#[test]
fn test_mem_leak2() {
    assert_eq!(Solution::mem_leak(8, 11), vec![6, 0, 4]);
}
