// Tests for Problem 2751: Robot Collisions
// Java reference: src/test/java/g2701_2800/s2751_robot_collisions/SolutionTest.java

use leetcode_in_rust::s2751::robot_collisions::Solution;

#[test]
fn test_survived_robots_healths() {
    let mut result = Solution::survived_robots_healths(
        vec![5, 4, 3, 2, 1],
        vec![2, 17, 9, 15, 10],
        "RRRRR".to_string(),
    );
    result.sort();
    assert_eq!(result, vec![2, 9, 10, 15, 17]);
}

#[test]
fn test_survived_robots_healths2() {
    assert_eq!(
        Solution::survived_robots_healths(
            vec![3, 5, 2, 6],
            vec![10, 10, 15, 12],
            "RLRL".to_string()
        ),
        vec![14]
    );
}

#[test]
fn test_survived_robots_healths3() {
    assert_eq!(
        Solution::survived_robots_healths(
            vec![1, 2, 5, 6],
            vec![10, 10, 11, 11],
            "RLRL".to_string()
        ),
        vec![] as Vec<i32>
    );
}

#[test]
fn test_survived_robots_healths4() {
    assert_eq!(
        Solution::survived_robots_healths(vec![1, 40], vec![10, 11], "RL".to_string()),
        vec![10]
    );
}
