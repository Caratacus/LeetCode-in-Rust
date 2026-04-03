// Tests for Problem 1431: Kids With the Greatest Number of Candies
// Java reference: src/test/java/g1401_1500/s1431_kids_with_the_greatest_number_of_candies/SolutionTest.java

use leetcode_in_rust::s1431::kids_with_the_greatest_number_of_candies::Solution;

#[test]
fn test_kids_with_candies() {
    assert_eq!(
        Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
        vec![true, true, true, false, true]
    );
}

#[test]
fn test_kids_with_candies2() {
    assert_eq!(
        Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
        vec![true, false, false, false, false]
    );
}

#[test]
fn test_kids_with_candies3() {
    assert_eq!(
        Solution::kids_with_candies(vec![12, 1, 12], 10),
        vec![true, false, true]
    );
}
