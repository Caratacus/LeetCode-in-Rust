// Tests for Problem 1884: Egg Drop With 2 Eggs and N Floors
// Java reference: src/test/java/g1801_1900/s1884_egg_drop_with_2_eggs_and_n_floors/SolutionTest.java

use leetcode_in_rust::s1884::egg_drop_with_2_eggs_and_n_floors::Solution;

#[test]
fn test_two_egg_drop() {
    assert_eq!(Solution::two_egg_drop(2), 2);
}

#[test]
fn test_two_egg_drop2() {
    assert_eq!(Solution::two_egg_drop(100), 14);
}
