// Tests for Problem 2873: Maximum Value of an Ordered Triplet I
// Java reference: src/test/java/g2801_2900/s2873_maximum_value_of_an_ordered_triplet_i/SolutionTest.java

use leetcode_in_rust::s2873::maximum_value_of_an_ordered_triplet_i::Solution;

#[test]
fn test_maximum_triplet_value() {
    assert_eq!(Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]), 77);
}

#[test]
fn test_maximum_triplet_value2() {
    assert_eq!(Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]), 133);
}

#[test]
fn test_maximum_triplet_value3() {
    assert_eq!(Solution::maximum_triplet_value(vec![1, 2, 3]), 0);
}

#[test]
fn test_maximum_triplet_value4() {
    assert_eq!(Solution::maximum_triplet_value(vec![8, 6, 3, 13, 2, 12, 19, 5, 19, 6, 10, 11, 9]), 266);
}
