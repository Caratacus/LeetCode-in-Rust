// Tests for Problem 0528: Random Pick with Weight
// Java reference: src/test/java/g0501_0600/s0528_random_pick_with_weight/SolutionTest.java

use leetcode_in_rust::s0528::random_pick_with_weight::Solution;

#[test]
fn test_solution_test() {
    // Note: Random test - just verify it returns valid index
    let idx = Solution::pick_index();
    assert_eq!(idx, 0);
}

#[test]
fn test_solution_test2() {
    // Note: Random test - just verify it returns valid index
    for _ in 0..5 {
        let idx = Solution::pick_index();
        assert!(idx == 0 || idx == 1);
    }
}
