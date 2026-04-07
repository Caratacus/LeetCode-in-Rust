// Tests for Problem 1893: Check if All the Integers in a Range Are Covered
// Java reference: src/test/java/g1801_1900/s1893_check_if_all_the_integers_in_a_range_are_covered/SolutionTest.java

use leetcode_in_rust::s1893::check_if_all_the_integers_in_a_range_are_covered::Solution;

#[test]
fn test_is_covered() {
    assert_eq!(
        Solution::is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5),
        true
    );
}

#[test]
fn test_is_covered2() {
    assert_eq!(
        Solution::is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21),
        false
    );
}
