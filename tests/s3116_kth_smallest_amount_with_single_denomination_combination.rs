// Tests for Problem 3116: Kth Smallest Amount With Single Denomination Combination
// Java reference: src/test/java/g3101_3200/s3116_kth_smallest_amount_with_single_denomination_combination/SolutionTest.java

use leetcode_in_rust::s3116::kth_smallest_amount_with_single_denomination_combination::Solution;

#[test]
fn test_find_kth_smallest() {
    assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
}

#[test]
fn test_find_kth_smallest2() {
    assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
}
