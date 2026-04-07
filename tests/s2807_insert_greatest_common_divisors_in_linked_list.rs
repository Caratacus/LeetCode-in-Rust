// Tests for Problem 2807: Insert Greatest Common Divisors in Linked List
// Java reference: src/test/java/g2701_2800/s2807_insert_greatest_common_divisors_in_linked_list/SolutionTest.java

use leetcode_in_rust::s2807::insert_greatest_common_divisors_in_linked_list::Solution;
use leetcode_in_rust::utils::linked_list_utils::{linked_list_from_vec, linked_list_to_vec};

#[test]
fn test_insert_greatest_common_divisors() {
    let input = linked_list_from_vec(vec![18, 6, 10, 3]);
    let result = Solution::insert_greatest_common_divisors(input);
    assert_eq!(linked_list_to_vec(result), vec![18, 6, 6, 2, 10, 1, 3]);
}

#[test]
fn test_insert_greatest_common_divisors2() {
    let input = linked_list_from_vec(vec![7]);
    let result = Solution::insert_greatest_common_divisors(input);
    assert_eq!(linked_list_to_vec(result), vec![7]);
}
