// Tests for Problem 1354: Construct Target Array With Multiple Sums
// Java reference: src/test/java/g1301_1400/s1354_construct_target_array_with_multiple_sums/SolutionTest.java

use leetcode_in_rust::s1354::construct_target_array_with_multiple_sums::Solution;

#[test]
fn test_is_possible() {
    let result = Solution::is_possible(vec![9, 3, 5]);
    assert_eq!(result, true);
}

#[test]
fn test_is_possible2() {
    let result = Solution::is_possible(vec![1, 1, 1, 2]);
    assert_eq!(result, false);
}

#[test]
fn test_is_possible3() {
    let result = Solution::is_possible(vec![8, 5]);
    assert_eq!(result, true);
}
