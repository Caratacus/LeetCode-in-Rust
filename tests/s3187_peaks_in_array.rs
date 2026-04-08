// Tests for Problem 3187: Peaks in Array
// Java reference: src/test/java/g3101_3200/s3187_peaks_in_array/SolutionTest.java

use leetcode_in_rust::s3187::peaks_in_array::Solution;

#[test]
fn test_count_of_peaks() {
    assert_eq!(
        Solution::count_of_peaks(vec![3, 1, 4, 2, 5], vec![vec![2, 3, 4], vec![1, 0, 4]]),
        vec![0]
    );
}

#[test]
fn test_count_of_peaks2() {
    assert_eq!(
        Solution::count_of_peaks(vec![4, 1, 4, 2, 1, 5], vec![vec![2, 2, 4], vec![1, 0, 2], vec![1, 0, 4]]),
        vec![0, 1]
    );
}
