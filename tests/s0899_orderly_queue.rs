// Tests for Problem 0899: Orderly Queue
// Java reference: src/test/java/g0801_0900/s0899_orderly_queue/SolutionTest.java

use leetcode_in_rust::s0899::orderly_queue::Solution;

#[test]
fn test_orderly_queue() {
    let result = Solution::orderly_queue("cba".to_string(), 1);
    assert_eq!(result, "acb");
}

#[test]
fn test_orderly_queue2() {
    let result = Solution::orderly_queue("baaca".to_string(), 3);
    assert_eq!(result, "aaabc");
}
