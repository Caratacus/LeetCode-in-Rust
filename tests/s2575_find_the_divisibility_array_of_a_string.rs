// Tests for Problem 2575: Find the Divisibility Array of a String
// Java reference: src/test/java/g2501_2600/s2575_find_the_divisibility_array_of_a_string/SolutionTest.java

use leetcode_in_rust::s2575::find_the_divisibility_array_of_a_string::Solution;

#[test]
fn test_divisibility_array() {
    assert_eq!(
        Solution::divisibility_array("998244353".to_string(), 3),
        vec![1, 1, 0, 0, 0, 1, 1, 0, 0]
    );
}

#[test]
fn test_divisibility_array2() {
    assert_eq!(
        Solution::divisibility_array("1010".to_string(), 10),
        vec![0, 1, 0, 1]
    );
}
