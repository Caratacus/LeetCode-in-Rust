use leetcode_in_rust::s0798::smallest_rotation_with_highest_score::Solution;

#[test]
fn test_best_rotation() {
    assert_eq!(Solution::best_rotation(vec![2, 3, 1, 4, 0]), 3);
}

#[test]
fn test_best_rotation2() {
    assert_eq!(Solution::best_rotation(vec![1, 3, 0, 2, 4]), 0);
}
