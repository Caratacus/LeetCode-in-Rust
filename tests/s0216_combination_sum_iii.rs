// Tests for Problem 0216: Combination Sum III
// Java reference: src/test/java/g0201_0300/s0216_combination_sum_iii/SolutionTest.java

use leetcode_in_rust::s0216::combination_sum_iii::Solution;

#[test]
fn test_combination_sum3() {
    let mut result = Solution::combination_sum3(3, 7);
    result.sort();
    assert_eq!(result, vec![vec![1, 2, 4]]);
}

#[test]
fn test_combination_sum32() {
    let mut result = Solution::combination_sum3(3, 9);
    result.sort();
    let mut expected = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
    expected.sort();
    assert_eq!(result, expected);
}
