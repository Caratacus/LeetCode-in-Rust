// Tests for Problem 2275: Largest Combination With Bitwise AND Greater Than Zero
// Java reference: src/test/java/g2201_2300/s2275_largest_combination_with_bitwise_and_greater_than_zero/SolutionTest.java

use leetcode_in_rust::s2275::largest_combination_with_bitwise_and_greater_than_zero::Solution;

#[test]
fn test_largest_combination() {
    assert_eq!(Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]), 4);
}

#[test]
fn test_largest_combination2() {
    assert_eq!(Solution::largest_combination(vec![8, 8]), 2);
}
