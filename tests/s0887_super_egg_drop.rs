// Tests for Problem 0887: Super Egg Drop
// Java reference: src/test/java/g0801_0900/s0887_super_egg_drop/SolutionTest.java

use leetcode_in_rust::s0887::super_egg_drop::Solution;

#[test]
fn test_super_egg_drop() {
    let result = Solution::super_egg_drop(1, 2);
    assert_eq!(result, 2);
}

#[test]
fn test_super_egg_drop2() {
    let result = Solution::super_egg_drop(2, 6);
    assert_eq!(result, 3);
}

#[test]
fn test_super_egg_drop3() {
    let result = Solution::super_egg_drop(3, 14);
    assert_eq!(result, 4);
}
