// Tests for Problem 3043: Find the Length of the Longest Common Prefix
// Java reference: src/test/java/g3001_3100/s3043_find_the_length_of_the_longest_common_prefix/SolutionTest.java

use leetcode_in_rust::s3043::find_the_length_of_the_longest_common_prefix::Solution;

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
        3
    );
}

#[test]
fn test_longest_common_prefix2() {
    assert_eq!(
        Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
        0
    );
}
