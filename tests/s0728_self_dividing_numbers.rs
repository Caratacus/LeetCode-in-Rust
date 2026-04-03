// Tests for Problem 0728: Self Dividing Numbers
// Java reference: src/test/java/g0701_0800/s0728_self_dividing_numbers/SolutionTest.java

use leetcode_in_rust::s0728::self_dividing_numbers::Solution;

#[test]
fn test_self_dividing_numbers() {
    assert_eq!(
        Solution::self_dividing_numbers(1, 22),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
}

#[test]
fn test_self_dividing_numbers2() {
    assert_eq!(
        Solution::self_dividing_numbers(47, 85),
        vec![48, 55, 66, 77]
    );
}
