// Tests for Problem 3393: Count Paths With the Given XOR Value
// Java reference: src/test/java/g3301_3400/s3393_count_paths_with_the_given_xor_value/SolutionTest.java

use leetcode_in_rust::s3393::count_paths_with_the_given_xor_value::Solution;

#[test]
fn test_count_paths_with_xor_value() {
    assert_eq!(
        Solution::count_paths_with_xor_value(
            vec![vec![2, 1, 5], vec![7, 10, 0], vec![12, 6, 4]],
            11
        ),
        3
    );
}

#[test]
fn test_count_paths_with_xor_value2() {
    assert_eq!(
        Solution::count_paths_with_xor_value(
            vec![vec![1, 3, 3, 3], vec![0, 3, 3, 2], vec![3, 0, 1, 1]],
            2
        ),
        5
    );
}

#[test]
fn test_count_paths_with_xor_value3() {
    assert_eq!(
        Solution::count_paths_with_xor_value(
            vec![vec![1, 1, 1, 2], vec![3, 0, 3, 2], vec![3, 0, 2, 2]],
            10
        ),
        0
    );
}
