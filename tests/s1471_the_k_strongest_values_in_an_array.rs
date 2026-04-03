// Tests for Problem 1471: The k Strongest Values in an Array
// Java reference: src/test/java/g1401_1500/s1471_the_k_strongest_values_in_an_array/SolutionTest.java

use leetcode_in_rust::s1471::the_k_strongest_values_in_an_array::Solution;

#[test]
fn test_get_strongest() {
    assert_eq!(Solution::get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
}

#[test]
fn test_get_strongest2() {
    assert_eq!(Solution::get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
}

#[test]
fn test_get_strongest3() {
    assert_eq!(Solution::get_strongest(vec![6, 7, 11, 7, 6, 8], 5), vec![11, 8, 6, 6, 7]);
}
