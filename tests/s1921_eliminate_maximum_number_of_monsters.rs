// Tests for Problem 1921: Eliminate Maximum Number of Monsters
// Java reference: src/test/java/g1901_2000/s1921_eliminate_maximum_number_of_monsters/SolutionTest.java

use leetcode_in_rust::s1921::eliminate_maximum_number_of_monsters::Solution;

#[test]
fn test_eliminate_maximum() {
    assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
}

#[test]
fn test_eliminate_maximum2() {
    assert_eq!(
        Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
        1
    );
}

#[test]
fn test_eliminate_maximum3() {
    assert_eq!(Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]), 1);
}
