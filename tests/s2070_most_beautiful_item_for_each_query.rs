// Tests for Problem 2070: Most Beautiful Item for Each Query
// Java reference: src/test/java/g2001_2100/s2070_most_beautiful_item_for_each_query/SolutionTest.java

use leetcode_in_rust::s2070::most_beautiful_item_for_each_query::Solution;

#[test]
fn test_maximum_beauty() {
    assert_eq!(
        Solution::maximum_beauty(
            vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
            vec![1, 2, 3, 4, 5, 6]
        ),
        vec![2, 4, 5, 5, 6, 6]
    );
}

#[test]
fn test_maximum_beauty2() {
    assert_eq!(
        Solution::maximum_beauty(
            vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
            vec![1]
        ),
        vec![4]
    );
}

#[test]
fn test_maximum_beauty3() {
    assert_eq!(
        Solution::maximum_beauty(vec![vec![10, 1000]], vec![5]),
        vec![0]
    );
}
