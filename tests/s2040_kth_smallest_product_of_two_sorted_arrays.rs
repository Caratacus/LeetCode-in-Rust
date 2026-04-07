// Tests for Problem 2040: Kth Smallest Product of Two Sorted Arrays
// Java reference: src/test/java/g2001_2100/s2040_kth_smallest_product_of_two_sorted_arrays/SolutionTest.java

use leetcode_in_rust::s2040::kth_smallest_product_of_two_sorted_arrays::Solution;

#[test]
fn test_kth_smallest_product() {
    assert_eq!(
        Solution::kth_smallest_product(vec![2, 5], vec![3, 4], 2),
        8
    );
}

#[test]
fn test_kth_smallest_product2() {
    assert_eq!(
        Solution::kth_smallest_product(vec![-4, -2, 0, 3], vec![2, 4], 6),
        0
    );
}

#[test]
fn test_kth_smallest_product3() {
    assert_eq!(
        Solution::kth_smallest_product(vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3),
        -6
    );
}
