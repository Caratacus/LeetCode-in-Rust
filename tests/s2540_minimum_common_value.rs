// Tests for Problem 2540: Minimum Common Value
// Java reference: src/test/java/g2501_2600/s2540_minimum_common_value/SolutionTest.java

use leetcode_in_rust::s2540::minimum_common_value::Solution;

#[test]
fn test_get_common() {
    assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
}
#[test]
fn test_get_common2() {
    assert_eq!(Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]), 2);
}
#[test]
fn test_get_common3() {
    assert_eq!(Solution::get_common(vec![1, 2, 3], vec![4, 5, 6]), -1);
}
#[test]
fn test_get_common4() {
    assert_eq!(Solution::get_common(vec![1, 3, 5, 7], vec![0, 2, 4, 7]), 7);
}
#[test]
fn test_get_common5() {
    assert_eq!(Solution::get_common(vec![2, 3, 4], vec![2, 5, 6]), 2);
}
