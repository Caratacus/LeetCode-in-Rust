// Tests for Problem 2444: Count Subarrays With Fixed Bounds
// Java reference: src/test/java/g2401_2500/s2444_count_subarrays_with_fixed_bounds/SolutionTest.java

use leetcode_in_rust::s2444::count_subarrays_with_fixed_bounds::Solution;

#[test]
fn test_count_subarrays() {
    assert_eq!(
        Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5),
        2
    );
}

#[test]
fn test_count_subarrays2() {
    assert_eq!(Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
}
