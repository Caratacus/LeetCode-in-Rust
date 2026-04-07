// Tests for Problem 1938: Maximum Genetic Difference Query
// Java reference: src/test/java/g1901_2000/s1938_maximum_genetic_difference_query/SolutionTest.java

use leetcode_in_rust::s1938::maximum_genetic_difference_query::Solution;

#[test]
fn test_max_genetic_difference() {
    assert_eq!(
        Solution::max_genetic_difference(vec![-1, 0, 1, 1], vec![vec![0, 2], vec![3, 2], vec![2, 5]]),
        vec![2, 3, 7]
    );
}

#[test]
fn test_max_genetic_difference2() {
    assert_eq!(
        Solution::max_genetic_difference(
            vec![3, 7, -1, 2, 0, 7, 0, 2],
            vec![vec![4, 6], vec![1, 15], vec![0, 5]]
        ),
        vec![7, 14, 7]
    );
}
