// Tests for Problem 0398: Random Pick Index
// Java reference: src/test/java/g0301_0400/s0398_random_pick_index/SolutionTest.java

use leetcode_in_rust::s0398::random_pick_index::Solution;

#[test]
fn test_pick() {
    let nums = vec![1, 2, 3, 3, 3];
    let solution = Solution::new(nums);
    // pick(3) should return one of indices 2, 3, or 4
    let index = solution.pick(3);
    assert!(index == 2 || index == 3 || index == 4);
    // pick(1) should return index 0
    assert_eq!(solution.pick(1), 0);
}
