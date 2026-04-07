// Tests for Problem 2334: Subarray With Elements Greater Than Varying Threshold
// Java reference: src/test/java/g2301_2400/s2334_subarray_with_elements_greater_than_varying_threshold/SolutionTest.java

use leetcode_in_rust::s2334::subarray_with_elements_greater_than_varying_threshold::Solution;

#[test]
fn test_valid_subarray_size() {
    assert_eq!(Solution::valid_subarray_size(vec![1, 3, 4, 3, 1], 6), 3);
}

#[test]
fn test_valid_subarray_size2() {
    assert_eq!(Solution::valid_subarray_size(vec![6, 5, 6, 5, 8], 7), 2);
}

#[test]
fn test_valid_subarray_size3() {
    assert_eq!(
        Solution::valid_subarray_size(
            vec![818, 232, 595, 418, 608, 229, 37, 330, 876, 774, 931, 939, 479, 884, 354, 328],
            3790
        ),
        -1
    );
}
