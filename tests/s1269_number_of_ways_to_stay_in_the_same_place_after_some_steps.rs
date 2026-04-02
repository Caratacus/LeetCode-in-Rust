// Tests for Problem 1269: Number of Ways to Stay in the Same Place After Some Steps
// Java reference: src/test/java/g1201_1300/s1269_number_of_ways_to_stay_in_the_same_place_after_some_steps/SolutionTest.java

use leetcode_in_rust::s1269::number_of_ways_to_stay_in_the_same_place_after_some_steps::Solution;

#[test]
fn test_num_ways() {
    assert_eq!(Solution::num_ways(3, 2), 4);
}

#[test]
fn test_num_ways2() {
    assert_eq!(Solution::num_ways(2, 4), 2);
}

#[test]
fn test_num_ways3() {
    assert_eq!(Solution::num_ways(4, 2), 8);
}
