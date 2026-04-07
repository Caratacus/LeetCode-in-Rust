// Tests for Problem 2240: Number of Ways to Buy Pens and Pencils
// Java reference: src/test/java/g2201_2300/s2240_number_of_ways_to_buy_pens_and_pencils/SolutionTest.java

use leetcode_in_rust::s2240::number_of_ways_to_buy_pens_and_pencils::Solution;

#[test]
fn test_ways_to_buy_pens_pencils() {
    assert_eq!(Solution::ways_to_buy_pens_pencils(20, 10, 5), 9);
}

#[test]
fn test_ways_to_buy_pens_pencils2() {
    assert_eq!(Solution::ways_to_buy_pens_pencils(5, 10, 10), 1);
}
