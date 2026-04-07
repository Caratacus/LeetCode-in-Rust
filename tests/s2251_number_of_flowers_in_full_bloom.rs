// Tests for Problem 2251: Number of Flowers in Full Bloom
// Java reference: src/test/java/g2201_2300/s2251_number_of_flowers_in_full_bloom/SolutionTest.java

use leetcode_in_rust::s2251::number_of_flowers_in_full_bloom::Solution;

#[test]
fn test_full_bloom_flowers() {
    assert_eq!(
        Solution::full_bloom_flowers(
            vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
            vec![2, 3, 7, 11]
        ),
        vec![1, 2, 2, 2]
    );
}

#[test]
fn test_full_bloom_flowers2() {
    assert_eq!(
        Solution::full_bloom_flowers(
            vec![vec![1, 10], vec![3, 3]],
            vec![3, 3, 2]
        ),
        vec![2, 2, 1]
    );
}
