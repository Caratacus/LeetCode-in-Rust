// Tests for Problem 1238: Circular Permutation in Binary Representation
// Java reference: src/test/java/g1201_1300/s1238_circular_permutation_in_binary_representation/SolutionTest.java

use leetcode_in_rust::s1238::circular_permutation_in_binary_representation::Solution;

fn is_valid_gray_code(result: &[i32], start: i32) -> bool {
    if result.is_empty() || result[0] != start {
        return false;
    }
    for i in 0..result.len() {
        let next = result[(i + 1) % result.len()];
        let xor = result[i] ^ next;
        if xor.count_ones() != 1 {
            return false;
        }
    }
    true
}

#[test]
fn test_circular_permutation() {
    let result = Solution::circular_permutation(2, 3);
    assert_eq!(result.len(), 4);
    assert!(is_valid_gray_code(&result, 3));
}

#[test]
fn test_circular_permutation2() {
    let result = Solution::circular_permutation(3, 2);
    assert_eq!(result.len(), 8);
    assert!(is_valid_gray_code(&result, 2));
}
