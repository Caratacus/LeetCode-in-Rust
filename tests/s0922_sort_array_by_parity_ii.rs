// Tests for Problem 0922: Sort Array By Parity II
// Java reference: src/test/java/g0901_1000/s0922_sort_array_by_parity_ii/SolutionTest.java

use leetcode_in_rust::s0922::sort_array_by_parity_ii::Solution;

#[test]
fn test_sort_array_by_parity_ii() {
    assert_eq!(Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]), vec![4, 5, 2, 7]);
}

#[test]
fn test_sort_array_by_parity_ii2() {
    assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
}
