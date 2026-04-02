// Tests for Problem 0858: Mirror Reflection
// Java reference: src/test/java/g0801_0900/s0858_mirror_reflection/SolutionTest.java

use leetcode_in_rust::s0858::mirror_reflection::Solution;

#[test]
fn test_mirror_reflection() {
    assert_eq!(Solution::mirror_reflection(2, 1), 2);
}

#[test]
fn test_mirror_reflection2() {
    assert_eq!(Solution::mirror_reflection(3, 1), 1);
}

#[test]
fn test_mirror_reflection3() {
    assert_eq!(Solution::mirror_reflection(4, 2), 2);
}

#[test]
fn test_mirror_reflection4() {
    assert_eq!(Solution::mirror_reflection(6, 3), 2);
}

#[test]
fn test_mirror_reflection5() {
    assert_eq!(Solution::mirror_reflection(5, 2), 0);
}

#[test]
fn test_mirror_reflection6() {
    assert_eq!(Solution::mirror_reflection(7, 3), 1);
}

#[test]
fn test_mirror_reflection7() {
    assert_eq!(Solution::mirror_reflection(1000, 250), 2);
}

#[test]
fn test_mirror_reflection8() {
    assert_eq!(Solution::mirror_reflection(4, 4), 1);
}

#[test]
fn test_mirror_reflection9() {
    assert_eq!(Solution::mirror_reflection(1, 1), 1);
}
