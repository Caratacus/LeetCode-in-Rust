// Tests for Problem 1343: Number of Sub-arrays of Size K and Average Greater Than or Equal to Threshold
// Java reference: src/test/java/g1301_1400/s1343_number_of_sub_arrays_of_size_k_and_average_greater_than_or_equal_to_threshold/SolutionTest.java

use leetcode_in_rust::s1343::number_of_sub_arrays_of_size_k_and_average_greater_than_or_equal_to_threshold::Solution;

#[test]
fn test_num_of_subarrays() {
    assert_eq!(Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
}

#[test]
fn test_num_of_subarrays2() {
    assert_eq!(Solution::num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5), 6);
}
