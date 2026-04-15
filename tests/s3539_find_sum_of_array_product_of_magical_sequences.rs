// Tests for Problem 3539: Find Sum of Array Product of Magical Sequences
// Java reference: src/test/java/g3501_3600/s3539_find_sum_of_array_product_of_magical_sequences/SolutionTest.java

use leetcode_in_rust::s3539::find_sum_of_array_product_of_magical_sequences::Solution;

#[test]
fn test_magical_sum() {
    assert_eq!(Solution::magical_sum(5, 5, vec![1, 10, 100, 10000, 1000000]), 991600007);
}

#[test]
fn test_magical_sum2() { assert_eq!(Solution::magical_sum(2, 2, vec![5, 4, 3, 2, 1]), 170); }

#[test]
fn test_magical_sum3() { assert_eq!(Solution::magical_sum(1, 1, vec![28]), 28); }
