// Tests for Problem 0907: Sum of Subarray Minimums
// Java reference: src/test/java/g0901_1000/s0907_sum_of_subarray_minimums/SolutionTest.java

use leetcode_in_rust::s0907::sum_of_subarray_minimums::Solution;

#[test]
fn test_sum_subarray_mins() {
    let result = Solution::sum_subarray_mins(vec![3, 1, 2, 4]);
    assert_eq!(result, 17);
}

#[test]
fn test_sum_subarray_mins2() {
    let result = Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]);
    assert_eq!(result, 444);
}
