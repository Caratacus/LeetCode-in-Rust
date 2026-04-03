// Tests for Problem 0805: Split Array With Same Average
// Java reference: src/test/java/g0701_0800/s0805_split_array_with_same_average/SolutionTest.java

use leetcode_in_rust::s0805::split_array_with_same_average::Solution;

#[test]
fn test_split_array_same_average() {
    assert_eq!(
        Solution::split_array_same_average(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        true
    );
}

#[test]
fn test_split_array_same_average2() {
    assert_eq!(Solution::split_array_same_average(vec![3, 1]), false);
}
