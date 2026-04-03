// Tests for Problem 0795: Number of Subarrays with Bounded Maximum
// Java reference: src/test/java/g0701_0800/s0795_number_of_subarrays_with_bounded_maximum/SolutionTest.java

use leetcode_in_rust::s0795::number_of_subarrays_with_bounded_maximum::Solution;

#[test]
fn test_num_subarray_bounded_max() {
    assert_eq!(
        Solution::num_subarray_bounded_max(vec![2, 1, 4, 3], 2, 3),
        3
    );
}

#[test]
fn test_num_subarray_bounded_max2() {
    assert_eq!(
        Solution::num_subarray_bounded_max(vec![2, 9, 2, 5, 6], 2, 8),
        7
    );
}
