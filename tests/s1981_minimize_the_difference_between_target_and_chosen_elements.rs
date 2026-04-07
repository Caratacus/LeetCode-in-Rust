// Tests for Problem 1981: Minimize the Difference Between Target and Chosen Elements
// Java reference: src/test/java/g1901_2000/s1981_minimize_the_difference_between_target_and_chosen_elements/SolutionTest.java

use leetcode_in_rust::s1981::minimize_the_difference_between_target_and_chosen_elements::Solution;

#[test]
fn test_minimize_the_difference() {
    assert_eq!(
        Solution::minimize_the_difference(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13),
        0
    );
}

#[test]
fn test_minimize_the_difference2() {
    assert_eq!(
        Solution::minimize_the_difference(vec![vec![1], vec![2], vec![3]], 100),
        94
    );
}

#[test]
fn test_minimize_the_difference3() {
    assert_eq!(
        Solution::minimize_the_difference(vec![vec![1, 2, 9, 8, 7]], 6),
        1
    );
}
