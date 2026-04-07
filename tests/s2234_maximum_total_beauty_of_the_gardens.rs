// Tests for Problem 2234: Maximum Total Beauty of the Gardens
// Java reference: src/test/java/g2201_2300/s2234_maximum_total_beauty_of_the_gardens/SolutionTest.java

use leetcode_in_rust::s2234::maximum_total_beauty_of_the_gardens::Solution;

#[test]
fn test_maximum_beauty() {
    assert_eq!(Solution::maximum_beauty(vec![1, 3, 1, 1], 7, 6, 12, 1), 14);
}

#[test]
fn test_maximum_beauty2() {
    assert_eq!(Solution::maximum_beauty(vec![2, 4, 5, 3], 10, 5, 2, 6), 30);
}
