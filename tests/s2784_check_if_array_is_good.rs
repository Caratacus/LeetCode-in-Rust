// Tests for Problem 2784: Check If Array is Good
// Java reference: src/test/java/g2701_2800/s2784_check_if_array_is_good/SolutionTest.java

use leetcode_in_rust::s2784::check_if_array_is_good::Solution;

#[test]
fn test_is_good() {
    assert_eq!(Solution::is_good(vec![2, 1, 3]), false);
}

#[test]
fn test_is_good2() {
    assert_eq!(Solution::is_good(vec![1, 3, 3, 2]), true);
}

#[test]
fn test_is_good3() {
    assert_eq!(Solution::is_good(vec![1, 1]), true);
}

#[test]
fn test_is_good4() {
    assert_eq!(Solution::is_good(vec![3, 4, 4, 1, 2, 1]), false);
}
