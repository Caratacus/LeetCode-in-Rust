// Tests for Problem 2525: Categorize Box According to Criteria
// Java reference: src/test/java/g2401_2500/s2525_categorize_box_according_to_criteria/SolutionTest.java

use leetcode_in_rust::s2525::categorize_box_according_to_criteria::Solution;

#[test]
fn test_categorize_box() {
    assert_eq!(Solution::categorize_box(1000, 35, 700, 300), "Heavy");
}

#[test]
fn test_categorize_box2() {
    assert_eq!(Solution::categorize_box(200, 50, 800, 50), "Neither");
}

#[test]
fn test_categorize_box3() {
    assert_eq!(Solution::categorize_box(10000, 1, 1, 10), "Bulky");
}

#[test]
fn test_categorize_box4() {
    assert_eq!(Solution::categorize_box(1000, 1000, 1000, 10), "Bulky");
}

#[test]
fn test_categorize_box5() {
    assert_eq!(Solution::categorize_box(10000, 10000, 1, 200), "Both");
}

#[test]
fn test_categorize_box6() {
    assert_eq!(Solution::categorize_box(9999, 9999, 1, 99), "Neither");
}

#[test]
fn test_categorize_box7() {
    assert_eq!(Solution::categorize_box(10000, 10000, 1, 100), "Both");
}

#[test]
fn test_categorize_box8() {
    assert_eq!(Solution::categorize_box(1000, 1000, 1000, 1), "Bulky");
}
