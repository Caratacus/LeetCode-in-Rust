// Tests for Problem 1224: Maximum Equal Frequency
// Java reference: src/test/java/g1201_1300/s1224_maximum_equal_frequency/SolutionTest.java

use leetcode_in_rust::s1224::maximum_equal_frequency::Solution;

#[test]
fn test_max_equal_freq() {
    assert_eq!(Solution::max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
}

#[test]
fn test_max_equal_freq2() {
    assert_eq!(
        Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]),
        13
    );
}
