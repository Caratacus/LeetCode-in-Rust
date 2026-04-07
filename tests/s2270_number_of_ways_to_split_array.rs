// Tests for Problem 2270: Number of Ways to Split Array
// Java reference: src/test/java/g2201_2300/s2270_number_of_ways_to_split_array/SolutionTest.java

use leetcode_in_rust::s2270::number_of_ways_to_split_array::Solution;

#[test]
fn test_ways_to_split_array() {
    assert_eq!(Solution::ways_to_split_array(vec![10, 4, -8, 7]), 2);
}

#[test]
fn test_ways_to_split_array2() {
    assert_eq!(Solution::ways_to_split_array(vec![2, 3, 1, 0]), 2);
}
