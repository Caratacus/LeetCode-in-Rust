// Tests for Problem 2588: Count the Number of Beautiful Subarrays
// Java reference: src/test/java/g2501_2600/s2588_count_the_number_of_beautiful_subarrays/SolutionTest.java

use leetcode_in_rust::s2588::count_the_number_of_beautiful_subarrays::Solution;

#[test]
fn test_beautiful_subarrays() {
    assert_eq!(Solution::beautiful_subarrays(vec![4, 3, 1, 2, 4]), 2);
}

#[test]
fn test_beautiful_subarrays2() {
    assert_eq!(Solution::beautiful_subarrays(vec![1, 10, 4]), 0);
}
