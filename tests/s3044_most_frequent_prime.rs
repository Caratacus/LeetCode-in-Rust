// Tests for Problem 3044: Most Frequent Prime
// Java reference: src/test/java/g3001_3100/s3044_most_frequent_prime/SolutionTest.java

use leetcode_in_rust::s3044::most_frequent_prime::Solution;

#[test]
fn test_most_frequent_prime() {
    let mat = vec![vec![1, 1], vec![9, 9], vec![1, 1]];
    assert_eq!(Solution::most_frequent_prime(mat), 19);
}

#[test]
fn test_most_frequent_prime2() {
    let mat = vec![vec![7]];
    assert_eq!(Solution::most_frequent_prime(mat), -1);
}

#[test]
fn test_most_frequent_prime3() {
    let mat = vec![vec![9, 7, 8], vec![4, 6, 5], vec![2, 8, 6]];
    assert_eq!(Solution::most_frequent_prime(mat), 97);
}
