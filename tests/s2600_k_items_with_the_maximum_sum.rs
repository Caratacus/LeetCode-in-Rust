// Tests for Problem 2600: K Items With the Maximum Sum
// Java reference: src/test/java/g2501_2600/s2600_k_items_with_the_maximum_sum/SolutionTest.java

use leetcode_in_rust::s2600::k_items_with_the_maximum_sum::Solution;

#[test]
fn test_k_items_with_maximum_sum() {
    assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 2), 2);
}

#[test]
fn test_k_items_with_maximum_sum2() {
    assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 4), 3);
}

#[test]
fn test_k_items_with_maximum_sum3() {
    assert_eq!(Solution::k_items_with_maximum_sum(5, 3, 2, 5), 5);
}

#[test]
fn test_k_items_with_maximum_sum4() {
    assert_eq!(Solution::k_items_with_maximum_sum(3, 4, 5, 7), 3);
}

#[test]
fn test_k_items_with_maximum_sum5() {
    assert_eq!(Solution::k_items_with_maximum_sum(3, 1, 5, 6), 1);
}

#[test]
fn test_k_items_with_maximum_sum6() {
    assert_eq!(Solution::k_items_with_maximum_sum(2, 1, 10, 13), -8);
}

#[test]
fn test_k_items_with_maximum_sum7() {
    assert_eq!(Solution::k_items_with_maximum_sum(0, 5, 5, 3), 0);
}

#[test]
fn test_k_items_with_maximum_sum8() {
    assert_eq!(Solution::k_items_with_maximum_sum(2, 0, 5, 3), 1);
}

#[test]
fn test_k_items_with_maximum_sum9() {
    assert_eq!(Solution::k_items_with_maximum_sum(4, 3, 0, 6), 4);
}

#[test]
fn test_k_items_with_maximum_sum10() {
    assert_eq!(Solution::k_items_with_maximum_sum(5, 5, 5, 0), 0);
}
