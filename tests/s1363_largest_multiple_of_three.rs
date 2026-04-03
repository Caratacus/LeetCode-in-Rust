// Tests for Problem 1363: Largest Multiple of Three
// Java reference: src/test/java/g1301_1400/s1363_largest_multiple_of_three/SolutionTest.java

use leetcode_in_rust::s1363::largest_multiple_of_three::Solution;

#[test]
fn test_largest_multiple_of_three() {
    let result = Solution::largest_multiple_of_three(vec![8, 1, 9]);
    assert_eq!(result, "981");
}

#[test]
fn test_largest_multiple_of_three2() {
    let result = Solution::largest_multiple_of_three(vec![8, 6, 7, 1, 0]);
    assert_eq!(result, "8760");
}

#[test]
fn test_largest_multiple_of_three3() {
    let result = Solution::largest_multiple_of_three(vec![1]);
    assert_eq!(result, "");
}
