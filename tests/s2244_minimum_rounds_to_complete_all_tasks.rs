// Tests for Problem 2244: Minimum Rounds to Complete All Tasks
// Java reference: src/test/java/g2201_2300/s2244_minimum_rounds_to_complete_all_tasks/SolutionTest.java

use leetcode_in_rust::s2244::minimum_rounds_to_complete_all_tasks::Solution;

#[test]
fn test_minimum_rounds() {
    assert_eq!(Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]), 4);
}

#[test]
fn test_minimum_rounds2() {
    assert_eq!(Solution::minimum_rounds(vec![2, 3, 3]), -1);
}

#[test]
fn test_minimum_rounds3() {
    assert_eq!(Solution::minimum_rounds(vec![2]), -1);
}

#[test]
fn test_minimum_rounds4() {
    assert_eq!(Solution::minimum_rounds(vec![4, 4, 4]), 1);
}
