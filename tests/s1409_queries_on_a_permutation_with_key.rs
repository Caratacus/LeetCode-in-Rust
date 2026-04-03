// Tests for Problem 1409: Queries on a Permutation With Key
// Java reference: src/test/java/g1301_1400/s1409_queries_on_a_permutation_with_key/SolutionTest.java

use leetcode_in_rust::s1409::queries_on_a_permutation_with_key::Solution;

#[test]
fn test_process_queries() {
    // queries=[3,1,2,1], m=5
    assert_eq!(Solution::process_queries(vec![3, 1, 2, 1], 5), vec![2, 1, 2, 1]);
}

#[test]
fn test_process_queries2() {
    // queries=[4,1,2,2], m=4
    assert_eq!(Solution::process_queries(vec![4, 1, 2, 2], 4), vec![3, 1, 2, 0]);
}

#[test]
fn test_process_queries3() {
    // queries=[7,5,5,8,3], m=8
    assert_eq!(Solution::process_queries(vec![7, 5, 5, 8, 3], 8), vec![6, 5, 0, 7, 5]);
}
