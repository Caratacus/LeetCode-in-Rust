// Tests for Problem 2197: Replace Non-Coprime Numbers in Array
// Java reference: src/test/java/g2101_2200/s2197_replace_non_coprime_numbers_in_array/SolutionTest.java

use leetcode_in_rust::s2197::replace_non_coprime_numbers_in_array::Solution;

#[test]
fn test_replace_non_coprimes() {
    assert_eq!(Solution::replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]), vec![12, 7, 6]);
}

#[test]
fn test_replace_non_coprimes2() {
    assert_eq!(Solution::replace_non_coprimes(vec![2, 2, 1, 1, 3, 3, 3]), vec![2, 1, 1, 3]);
}

#[test]
fn test_replace_non_coprimes3() {
    assert_eq!(
        Solution::replace_non_coprimes(vec![287, 41, 49, 287, 899, 23, 23, 20677, 5, 825]),
        vec![2009, 20677, 825]
    );
}
