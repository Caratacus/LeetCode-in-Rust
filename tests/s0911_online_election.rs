// Tests for Problem 0911: Online Election
// Java reference: src/test/java/g0901_1000/s0911_online_election/SolutionTest.java

use leetcode_in_rust::s0911::online_election::TopVotedCandidate;

#[test]
fn test_top_voted_candidate_test() {
    let mut candidate = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
    assert_eq!(candidate.q(3), 0);
    assert_eq!(candidate.q(12), 1);
    assert_eq!(candidate.q(25), 1);
    assert_eq!(candidate.q(15), 0);
    assert_eq!(candidate.q(24), 0);
    assert_eq!(candidate.q(8), 1);
}
