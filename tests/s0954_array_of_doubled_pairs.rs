// Tests for Problem 0954: Array of Doubled Pairs
// Java reference: src/test/java/g0901_1000/s0954_array_of_doubled_pairs/SolutionTest.java

use leetcode_in_rust::s0954::array_of_doubled_pairs::Solution;

#[test]
fn test_can_reorder_doubled() {
    let result = Solution::can_reorder_doubled(vec![3, 1, 3, 6]);
    assert_eq!(result, false);
}

#[test]
fn test_can_reorder_doubled2() {
    let result = Solution::can_reorder_doubled(vec![2, 1, 2, 6]);
    assert_eq!(result, false);
}

#[test]
fn test_can_reorder_doubled3() {
    let result = Solution::can_reorder_doubled(vec![4, -2, 2, -4]);
    assert_eq!(result, true);
}
