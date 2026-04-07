// Tests for Problem 1851: Minimum Interval to Include Each Query
// Java reference: src/test/java/g1801_1900/s1851_minimum_interval_to_include_each_query/SolutionTest.java

use leetcode_in_rust::s1851::minimum_interval_to_include_each_query::Solution;

#[test]
fn test_min_interval() {
    assert_eq!(
        Solution::min_interval(
            vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]],
            vec![2, 3, 4, 5]
        ),
        vec![3, 3, 1, 4]
    );
}

#[test]
fn test_min_interval2() {
    assert_eq!(
        Solution::min_interval(
            vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]],
            vec![2, 19, 5, 22]
        ),
        vec![2, -1, 4, 6]
    );
}
