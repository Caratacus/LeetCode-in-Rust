// Tests for Problem 1997: First Day Where You Have Been in All the Rooms
// Java reference: src/test/java/g1901_2000/s1997_first_day_where_you_have_been_in_all_the_rooms/SolutionTest.java

use leetcode_in_rust::s1997::first_day_where_you_have_been_in_all_the_rooms::Solution;

#[test]
fn test_first_day_been_in_all_rooms() {
    assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0]), 2);
}

#[test]
fn test_first_day_been_in_all_rooms2() {
    assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0, 2]), 6);
}

#[test]
fn test_first_day_been_in_all_rooms3() {
    assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 1, 2, 0]), 6);
}
