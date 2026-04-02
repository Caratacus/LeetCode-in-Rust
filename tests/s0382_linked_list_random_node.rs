// Tests for Problem 0382: Linked List Random Node
// Java reference: src/test/java/g0301_0400/s0382_linked_list_random_node/SolutionTest.java

use leetcode_in_rust::s0382::linked_list_random_node::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_get_random() {
    let head = linked_list_from_vec(vec![1, 2, 3]);
    let solution = Solution::new(head);
    // getRandom should return either 1, 2, or 3
    let rand_val = solution.get_random();
    assert!(rand_val == 1 || rand_val == 2 || rand_val == 3);
}

#[test]
fn test_get_random2() {
    let head = linked_list_from_vec(vec![1]);
    let solution = Solution::new(head);
    // getRandom should always return 1
    assert_eq!(solution.get_random(), 1);
}
