// Tests for Problem 2206: Divide Array Into Equal Pairs
// Java reference: src/test/java/g2201_2300/s2206_divide_array_into_equal_pairs/SolutionTest.java

use leetcode_in_rust::s2206::divide_array_into_equal_pairs::Solution;

#[test]
fn test_divide_array() {
    assert_eq!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]), true);
}

#[test]
fn test_divide_array2() {
    assert_eq!(Solution::divide_array(vec![1, 2, 3, 4]), false);
}
