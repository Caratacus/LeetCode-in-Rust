// Tests for Problem 1518: Water Bottles
// Java reference: src/test/java/g1501_1600/s1518_water_bottles/SolutionTest.java

use leetcode_in_rust::s1518::water_bottles::Solution;

#[test]
fn test_num_water_bottles() {
    assert_eq!(Solution::num_water_bottles(9, 3), 13);
}

#[test]
fn test_num_water_bottles2() {
    assert_eq!(Solution::num_water_bottles(15, 4), 19);
}
