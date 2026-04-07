// Tests for Problem 2210: Count Hills and Valleys in an Array
// Java reference: src/test/java/g2201_2300/s2210_count_hills_and_valleys_in_an_array/SolutionTest.java

use leetcode_in_rust::s2210::count_hills_and_valleys_in_an_array::Solution;

#[test]
fn test_count_hill_valley() {
    assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
}

#[test]
fn test_count_hill_valley2() {
    assert_eq!(Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
}
