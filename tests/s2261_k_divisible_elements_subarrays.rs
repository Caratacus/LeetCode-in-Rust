// Tests for Problem 2261: K Divisible Elements Subarrays
// Java reference: src/test/java/g2201_2300/s2261_k_divisible_elements_subarrays/SolutionTest.java

use leetcode_in_rust::s2261::k_divisible_elements_subarrays::Solution;

#[test]
fn test_count_distinct() {
    assert_eq!(Solution::count_distinct(vec![2, 3, 3, 2, 2], 2, 2), 11);
}

#[test]
fn test_count_distinct2() {
    assert_eq!(Solution::count_distinct(vec![1, 2, 3, 4], 4, 1), 10);
}
