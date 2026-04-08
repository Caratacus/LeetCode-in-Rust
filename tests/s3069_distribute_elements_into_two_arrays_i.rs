// Tests for Problem 3069: Distribute Elements Into Two Arrays I
// Java reference: src/test/java/g3001_3100/s3069_distribute_elements_into_two_arrays_i/SolutionTest.java

use leetcode_in_rust::s3069::distribute_elements_into_two_arrays_i::Solution;

#[test]
fn test_result_array() {
    assert_eq!(Solution::result_array(vec![2, 1, 3]), vec![2, 3, 1]);
}

#[test]
fn test_result_array2() {
    assert_eq!(
        Solution::result_array(vec![5, 4, 3, 8]),
        vec![5, 3, 4, 8]
    );
}
