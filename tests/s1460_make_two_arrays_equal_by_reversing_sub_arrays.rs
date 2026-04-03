// Tests for Problem 1460: Make Two Arrays Equal by Reversing Sub-arrays
// Java reference: src/test/java/g1401_1500/s1460_make_two_arrays_equal_by_reversing_sub_arrays/SolutionTest.java

use leetcode_in_rust::s1460::make_two_arrays_equal_by_reversing_sub_arrays::Solution;

#[test]
fn test_can_be_equal() {
    assert_eq!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
}

#[test]
fn test_can_be_equal2() {
    assert_eq!(Solution::can_be_equal(vec![7], vec![7]), true);
}

#[test]
fn test_can_be_equal3() {
    assert_eq!(Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
}
