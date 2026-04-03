// Tests for Problem 1375: Number of Times Binary String is Prefix Aligned
// Java reference: src/test/java/g1301_1400/s1375_number_of_times_binary_string_is_prefix_aligned/SolutionTest.java

use leetcode_in_rust::s1375::number_of_times_binary_string_is_prefix_aligned::Solution;

#[test]
fn test_num_times_all_blue() {
    assert_eq!(Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]), 2);
}

#[test]
fn test_num_times_all_blue2() {
    assert_eq!(Solution::num_times_all_blue(vec![4, 1, 2, 3]), 1);
}
