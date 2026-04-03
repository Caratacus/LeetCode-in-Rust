// Tests for Problem 1539: Kth Missing Positive Number
// Java reference: src/test/java/g1501_1600/s1539_kth_missing_positive_number/SolutionTest.java

use leetcode_in_rust::s1539::kth_missing_positive_number::Solution;

#[test]
fn test_find_kth_positive() {
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
}

#[test]
fn test_find_kth_positive2() {
    assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
}
