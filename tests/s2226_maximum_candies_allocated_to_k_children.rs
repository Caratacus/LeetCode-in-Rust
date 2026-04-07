// Tests for Problem 2226: Maximum Candies Allocated to K Children
// Java reference: src/test/java/g2201_2300/s2226_maximum_candies_allocated_to_k_children/SolutionTest.java

use leetcode_in_rust::s2226::maximum_candies_allocated_to_k_children::Solution;

#[test]
fn test_maximum_candies() {
    assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5);
}

#[test]
fn test_maximum_candies2() {
    assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
}

#[test]
fn test_maximum_candies3() {
    assert_eq!(Solution::maximum_candies(vec![1, 2, 3, 4, 10], 5), 3);
}
