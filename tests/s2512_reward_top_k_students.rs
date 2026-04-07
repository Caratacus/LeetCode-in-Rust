// Tests for Problem 2512: Reward Top K Students
// Java reference: src/test/java/g2401_2500/s2512_reward_top_k_students/SolutionTest.java

use leetcode_in_rust::s2512::reward_top_k_students::Solution;

#[test]
fn test_top_students() {
    // Note: Implementation uses todo!() - test will fail until implemented
    let result = Solution::top_students(
        vec!["smart".to_string(), "brilliant".to_string(), "studious".to_string()],
        vec!["not".to_string()],
        vec!["this student is studious".to_string(), "the student is smart".to_string()],
        vec![1, 2],
        2
    );
    assert_eq!(result, vec![1, 2]);
}
