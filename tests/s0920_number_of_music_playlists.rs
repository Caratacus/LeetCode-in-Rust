// Tests for Problem 0920: Number of Music Playlists
// Java reference: src/test/java/g0901_1000/s0920_number_of_music_playlists/SolutionTest.java

use leetcode_in_rust::s0920::number_of_music_playlists::Solution;

#[test]
fn test_num_music_playlists() {
    let result = Solution::num_music_playlists(3, 3, 1);
    assert_eq!(result, 6);
}

#[test]
fn test_num_music_playlists2() {
    let result = Solution::num_music_playlists(2, 3, 0);
    assert_eq!(result, 6);
}

#[test]
fn test_num_music_playlists3() {
    let result = Solution::num_music_playlists(2, 3, 1);
    assert_eq!(result, 2);
}
