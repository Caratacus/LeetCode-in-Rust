// Tests for Problem 2550: Count Collisions of Monkeys on a Polygon
// Java reference: src/test/java/g2501_2600/s2550_count_collisions_of_monkeys_on_a_polygon/SolutionTest.java
use leetcode_in_rust::s2550::count_collisions_of_monkeys_on_a_polygon::Solution;

#[test]
fn test_monkey_move() {
    assert_eq!(Solution::monkey_move(3), 6);
}
#[test]
fn test_monkey_move2() {
    assert_eq!(Solution::monkey_move(4), 14);
}
