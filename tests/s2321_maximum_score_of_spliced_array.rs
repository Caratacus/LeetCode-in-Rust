// Tests for Problem 2321: Maximum Score of Spliced Array
// Java reference: src/test/java/g2301_2400/s2321_maximum_score_of_spliced_array/SolutionTest.java

use leetcode_in_rust::s2321::maximum_score_of_spliced_array::Solution;

#[test]
fn test_maximums_spliced_array() {
    assert_eq!(
        Solution::maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10]),
        210
    );
}

#[test]
fn test_maximums_spliced_array2() {
    assert_eq!(
        Solution::maximums_spliced_array(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]),
        220
    );
}

#[test]
fn test_maximums_spliced_array3() {
    assert_eq!(Solution::maximums_spliced_array(vec![7, 11, 13], vec![1, 1, 1]), 31);
}

#[test]
fn test_maximums_spliced_array4() {
    assert_eq!(Solution::maximums_spliced_array(vec![1, 1, 1], vec![7, 11, 13]), 31);
}
