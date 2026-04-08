// Tests for Problem 3264: Final Array State After K Multiplication Operations I
// Java reference: src/test/java/g3201_3300/s3264_final_array_state_after_k_multiplication_operations_i/SolutionTest.java

use leetcode_in_rust::s3264::final_array_state_after_k_multiplication_operations_i::Solution;

#[test]
fn test_get_final_state() {
    assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), vec![16, 8]);
}
