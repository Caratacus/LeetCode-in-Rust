// Tests for Problem 3266: Final Array State After K Multiplication Operations II
// Java reference: src/test/java/g3201_3300/s3266_final_array_state_after_k_multiplication_operations_ii/SolutionTest.java

use leetcode_in_rust::s3266::final_array_state_after_k_multiplication_operations_ii::Solution;

#[test]
fn test_get_final_state() {
    assert_eq!(
        Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
        vec![8, 4, 6, 5, 6]
    );
}

#[test]
fn test_get_final_state2() {
    assert_eq!(
        Solution::get_final_state(vec![100000, 2000], 2, 1000000),
        vec![999999307, 999999993]
    );
}
