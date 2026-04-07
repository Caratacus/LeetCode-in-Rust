// Tests for Problem 2810: Faulty Keyboard
// Java reference: src/test/java/g2801_2900/s2810_faulty_keyboard/SolutionTest.java

use leetcode_in_rust::s2810::faulty_keyboard::Solution;

#[test]
fn test_final_string() {
    assert_eq!(Solution::final_string("string".to_string()), "rtsng");
}

#[test]
fn test_final_string2() {
    assert_eq!(Solution::final_string("poiinter".to_string()), "ponter");
}
