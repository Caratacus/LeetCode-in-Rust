// Tests for Problem 1712: Ways to Split Array Into Three Subarrays
// Java reference: src/test/java/g1701_1800/s1712_ways_to_split_array_into_three_subarrays/SolutionTest.java

use leetcode_in_rust::s1712::ways_to_split_array_into_three_subarrays::Solution;

#[test]
fn test_ways_to_split() {
    assert_eq!(Solution::ways_to_split(vec![1, 1, 1]), 1);
}

#[test]
fn test_ways_to_split2() {
    assert_eq!(Solution::ways_to_split(vec![1, 2, 2, 2, 5, 0]), 3);
}

#[test]
fn test_ways_to_split3() {
    assert_eq!(Solution::ways_to_split(vec![3, 2, 1]), 0);
}
