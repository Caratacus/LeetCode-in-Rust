// Tests for Problem 1338: Reduce Array Size to The Half
// Java reference: src/test/java/g1301_1400/s1338_reduce_array_size_to_the_half/SolutionTest.java

use leetcode_in_rust::s1338::reduce_array_size_to_the_half::Solution;

#[test]
fn test_min_set_size() {
    assert_eq!(Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]), 2);
}

#[test]
fn test_min_set_size2() {
    assert_eq!(Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
}
