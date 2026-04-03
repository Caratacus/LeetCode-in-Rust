// Tests for Problem 1434: Number of Ways to Wear Different Hats to Each Other
// Java reference: src/test/java/g1401_1500/s1434_number_of_ways_to_wear_different_hats_to_each_other/SolutionTest.java

use leetcode_in_rust::s1434::number_of_ways_to_wear_different_hats_to_each_other::Solution;

#[test]
fn test_number_ways() {
    let hats = vec![
        vec![3, 4],
        vec![4, 5],
        vec![5],
    ];
    assert_eq!(Solution::number_ways(hats), 1);
}

#[test]
fn test_number_ways2() {
    let hats = vec![
        vec![3, 5, 1],
        vec![3, 5],
    ];
    assert_eq!(Solution::number_ways(hats), 4);
}

#[test]
fn test_number_ways3() {
    let hats = vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
    ];
    assert_eq!(Solution::number_ways(hats), 24);
}
