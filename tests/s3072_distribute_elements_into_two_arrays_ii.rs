// Tests for Problem 3072: Distribute Elements Into Two Arrays II
// Java reference: src/test/java/g3001_3100/s3072_distribute_elements_into_two_arrays_ii/SolutionTest.java

use leetcode_in_rust::s3072::distribute_elements_into_two_arrays_ii::Solution;

#[test]
fn test_result_array() {
    assert_eq!(
        Solution::result_array(vec![2, 1, 3, 3]),
        vec![2, 3, 1, 3]
    );
}

#[test]
fn test_result_array2() {
    assert_eq!(
        Solution::result_array(vec![5, 14, 3, 1, 2]),
        vec![5, 3, 2, 14, 1]
    );
}

#[test]
fn test_result_array3() {
    assert_eq!(
        Solution::result_array(vec![3, 3, 3, 3]),
        vec![3, 3, 3, 3]
    );
}
