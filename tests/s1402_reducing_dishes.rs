// Tests for Problem 1402: Reducing Dishes
// Java reference: src/test/java/g1301_1400/s1402_reducing_dishes/SolutionTest.java

use leetcode_in_rust::s1402::reducing_dishes::Solution;

#[test]
fn test_max_satisfaction() {
    assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
}

#[test]
fn test_max_satisfaction2() {
    assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
}

#[test]
fn test_max_satisfaction3() {
    assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
}
