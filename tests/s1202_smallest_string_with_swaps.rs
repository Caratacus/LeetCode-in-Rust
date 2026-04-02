// Tests for Problem 1202: Smallest String With Swaps
// Java reference: src/test/java/g1201_1300/s1202_smallest_string_with_swaps/SolutionTest.java

use leetcode_in_rust::s1202::smallest_string_with_swaps::Solution;

#[test]
fn test_smallest_string_with_swaps() {
    assert_eq!(
        Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]]),
        "bacd"
    );
}

#[test]
fn test_smallest_string_with_swaps2() {
    assert_eq!(
        Solution::smallest_string_with_swaps(
            "dcab".to_string(),
            vec![vec![0, 3], vec![1, 2], vec![0, 2]]
        ),
        "abcd"
    );
}

#[test]
fn test_smallest_string_with_swaps3() {
    assert_eq!(
        Solution::smallest_string_with_swaps("cba".to_string(), vec![vec![0, 1], vec![1, 2]]),
        "abc"
    );
}
