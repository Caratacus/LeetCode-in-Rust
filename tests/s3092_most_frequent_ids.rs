// Tests for Problem 3092: Most Frequent IDs
// Java reference: src/test/java/g3001_3100/s3092_most_frequent_ids/SolutionTest.java

use leetcode_in_rust::s3092::most_frequent_ids::Solution;

#[test]
fn test_most_frequent_i_ds() {
    assert_eq!(
        Solution::most_frequent_i_ds(vec![2, 3, 2, 1], vec![3, 2, -3, 1]),
        vec![3, 3, 2, 2]
    );
}

#[test]
fn test_most_frequent_i_ds2() {
    assert_eq!(
        Solution::most_frequent_i_ds(vec![5, 5, 3], vec![2, -2, 1]),
        vec![2, 0, 1]
    );
}
