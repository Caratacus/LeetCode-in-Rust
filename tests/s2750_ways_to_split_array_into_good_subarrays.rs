// Tests for Problem 2750: Ways to Split Array Into Good Subarrays
// Java reference: src/test/java/g2701_2800/s2750_ways_to_split_array_into_good_subarrays/SolutionTest.java

use leetcode_in_rust::s2750::ways_to_split_array_into_good_subarrays::Solution;

#[test]
fn test_number_of_good_subarray_splits() {
    assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0, 1]), 1);
}

#[test]
fn test_number_of_good_subarray_splits2() {
    assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 1, 0]), 1);
}

#[test]
fn test_number_of_good_subarray_splits3() {
    assert_eq!(Solution::number_of_good_subarray_splits(vec![0, 0]), 0);
}
