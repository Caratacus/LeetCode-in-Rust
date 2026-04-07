// Tests for Problem 2279: Maximum Bags With Full Capacity of Rocks
// Java reference: src/test/java/g2201_2300/s2279_maximum_bags_with_full_capacity_of_rocks/SolutionTest.java

use leetcode_in_rust::s2279::maximum_bags_with_full_capacity_of_rocks::Solution;

#[test]
fn test_maximum_bags() {
    assert_eq!(Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2), 3);
}

#[test]
fn test_maximum_bags2() {
    assert_eq!(Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100), 3);
}

#[test]
fn test_maximum_bags3() {
    assert_eq!(
        Solution::maximum_bags(vec![91, 54, 63, 99, 24, 45, 78], vec![35, 32, 45, 98, 6, 1, 25], 17),
        1
    );
}
