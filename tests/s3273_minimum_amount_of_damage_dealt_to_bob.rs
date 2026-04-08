// Tests for Problem 3273: Minimum Amount of Damage Dealt to Bob
// Java reference: src/test/java/g3201_3300/s3273_minimum_amount_of_damage_dealt_to_bob/SolutionTest.java

use leetcode_in_rust::s3273::minimum_amount_of_damage_dealt_to_bob::Solution;

#[test]
fn test_min_damage() {
    assert_eq!(Solution::min_damage(4, vec![1, 2, 3, 4], vec![4, 5, 6, 8]), 39);
}

#[test]
fn test_min_damage2() {
    assert_eq!(Solution::min_damage(1, vec![1, 1, 1, 1], vec![1, 2, 3, 4]), 20);
}

#[test]
fn test_min_damage3() {
    assert_eq!(Solution::min_damage(8, vec![40], vec![59]), 320);
}
