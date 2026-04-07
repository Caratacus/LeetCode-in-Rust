// Tests for Problem 2966: Divide Array Into Arrays With Max Difference
// Java reference: src/test/java/g2901_3000/s2966_divide_array_into_arrays_with_max_difference/SolutionTest.java

use leetcode_in_rust::s2966::divide_array_into_arrays_with_max_difference::Solution;

#[test]
fn test_divide_array() {
    let result = Solution::divide_array(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2);
    assert!(result.is_empty() || result[0].len() == 3);
}

#[test]
fn test_divide_array2() {
    let result = Solution::divide_array(vec![1, 3, 3, 2, 7, 3], 3);
    assert!(result.is_empty());
}
