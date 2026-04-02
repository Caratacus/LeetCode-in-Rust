// Tests for Problem 1248: Count Number of Nice Subarrays
// Java reference: src/test/java/g1201_1300/s1248_count_number_of_nice_subarrays/SolutionTest.java

use leetcode_in_rust::s1248::count_number_of_nice_subarrays::Solution;

#[test]
fn test_number_of_subarrays() {
    assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
}

#[test]
fn test_number_of_subarrays2() {
    assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
}

#[test]
fn test_number_of_subarrays3() {
    assert_eq!(
        Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
        16
    );
}
