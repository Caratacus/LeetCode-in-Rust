// Tests for Problem 1276: Number of Burgers with No Waste of Ingredients
// Java reference: src/test/java/g1201_1300/s1276_number_of_burgers_with_no_waste_of_ingredients/SolutionTest.java

use leetcode_in_rust::s1276::number_of_burgers_with_no_waste_of_ingredients::Solution;

#[test]
fn test_num_of_burgers() {
    assert_eq!(Solution::num_of_burgers(16, 7), vec![1, 6]);
}

#[test]
fn test_num_of_burgers2() {
    assert_eq!(Solution::num_of_burgers(17, 4), vec![]);
}

#[test]
fn test_num_of_burgers3() {
    assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
}
