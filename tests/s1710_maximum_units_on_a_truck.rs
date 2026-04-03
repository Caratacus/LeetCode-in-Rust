// Tests for Problem 1710: Maximum Units on a Truck
// Java reference: src/test/java/g1701_1800/s1710_maximum_units_on_a_truck/SolutionTest.java

use leetcode_in_rust::s1710::maximum_units_on_a_truck::Solution;

#[test]
fn test_maximum_units() {
    assert_eq!(
        Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
        8
    );
}

#[test]
fn test_maximum_units2() {
    assert_eq!(
        Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
        91
    );
}
