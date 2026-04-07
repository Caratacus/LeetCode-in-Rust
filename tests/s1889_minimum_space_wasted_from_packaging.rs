// Tests for Problem 1889: Minimum Space Wasted From Packaging
// Java reference: src/test/java/g1801_1900/s1889_minimum_space_wasted_from_packaging/SolutionTest.java

use leetcode_in_rust::s1889::minimum_space_wasted_from_packaging::Solution;

#[test]
fn test_min_wasted_space() {
    assert_eq!(
        Solution::min_wasted_space(vec![2, 3, 5], vec![vec![4, 8], vec![2, 8]]),
        6
    );
}

#[test]
fn test_min_wasted_space2() {
    assert_eq!(
        Solution::min_wasted_space(vec![2, 3, 5], vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        -1
    );
}

#[test]
fn test_min_wasted_space3() {
    assert_eq!(
        Solution::min_wasted_space(
            vec![3, 5, 8, 10, 11, 12],
            vec![vec![12], vec![11, 9], vec![10, 5, 14]]
        ),
        9
    );
}
