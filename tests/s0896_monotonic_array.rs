// Tests for Problem 0896: Monotonic Array
// Java reference: src/test/java/g0801_0900/s0896_monotonic_array/SolutionTest.java

use leetcode_in_rust::s0896::monotonic_array::Solution;

#[test]
fn test_is_monotonic() {
    let result = Solution::is_monotonic(vec![1, 2, 2, 3]);
    assert_eq!(result, true);
}

#[test]
fn test_is_monotonic2() {
    let result = Solution::is_monotonic(vec![6, 5, 4, 4]);
    assert_eq!(result, true);
}

#[test]
fn test_is_monotonic3() {
    let result = Solution::is_monotonic(vec![1, 3, 2]);
    assert_eq!(result, false);
}
