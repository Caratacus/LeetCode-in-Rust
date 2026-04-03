// Tests for Problem 1330: Reverse Subarray To Maximize Array Value
// Java reference: src/test/java/g1301_1400/s1330_reverse_subarray_to_maximize_array_value/SolutionTest.java

use leetcode_in_rust::s1330::reverse_subarray_to_maximize_array_value::Solution;

#[test]
fn test_max_value_after_reverse() {
    let result = Solution::max_value_after_reverse(vec![2, 3, 1, 5, 4]);
    assert_eq!(result, 10);
}

#[test]
fn test_max_value_after_reverse2() {
    let result = Solution::max_value_after_reverse(vec![2, 4, 9, 24, 2, 1, 10]);
    assert_eq!(result, 68);
}
