// Tests for Problem 3186: Maximum Total Damage With Spell Casting
// Java reference: src/test/java/g3101_3200/s3186_maximum_total_damage_with_spell_casting/SolutionTest.java

use leetcode_in_rust::s3186::maximum_total_damage_with_spell_casting::Solution;

#[test]
fn test_maximum_total_damage() {
    assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
}

#[test]
fn test_maximum_total_damage2() {
    assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 6]), 13);
}

#[test]
fn test_maximum_total_damage3() {
    assert_eq!(Solution::maximum_total_damage(vec![1_000_001, 1, 6, 6]), 1_000_014);
}
