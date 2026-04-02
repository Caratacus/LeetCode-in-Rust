// Tests for Problem 0384: Shuffle an Array
// Java reference: src/test/java/g0301_0400/s0384_shuffle_an_array/SolutionTest.java

use leetcode_in_rust::s0384::shuffle_an_array::Solution;

#[test]
fn test_shuffle_and_reset() {
    let nums = vec![1, 2, 3];
    let mut solution = Solution::new(nums.clone());
    let shuffled = solution.shuffle();
    // After sorting, shuffled should equal original
    let mut sorted_shuffled = shuffled.clone();
    sorted_shuffled.sort();
    assert_eq!(sorted_shuffled, nums);
    // reset should return original array
    assert_eq!(solution.reset(), nums);
}

#[test]
fn test_shuffle_and_reset2() {
    let nums = vec![1];
    let mut solution = Solution::new(nums.clone());
    assert_eq!(solution.shuffle(), nums);
    assert_eq!(solution.reset(), nums);
}
