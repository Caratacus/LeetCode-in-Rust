// Tests for Problem 0997: Find the Town Judge
// Java reference: src/test/java/g0901_1000/s0997_find_the_town_judge/SolutionTest.java

use leetcode_in_rust::s0997::find_the_town_judge::Solution;

#[test]
fn test_find_judge() {
    assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
}

#[test]
fn test_find_judge2() {
    assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
}
