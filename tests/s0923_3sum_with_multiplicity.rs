// Tests for Problem 0923: 3Sum With Multiplicity
// Java reference: src/test/java/g0901_1000/s0923_3sum_with_multiplicity/SolutionTest.java

use leetcode_in_rust::s0923::p3sum_with_multiplicity::Solution;

#[test]
fn test_three_sum_multi() {
    let result = Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8);
    assert_eq!(result, 20);
}

#[test]
fn test_three_sum_multi2() {
    let result = Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5);
    assert_eq!(result, 12);
}
