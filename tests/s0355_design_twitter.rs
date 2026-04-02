// Tests for Problem 0355: Design Twitter
// Java reference: src/test/java/g0301_0400/s0355_design_twitter/SolutionTest.java

use leetcode_in_rust::s0355::design_twitter::Twitter;

#[test]
fn test_twitter() {
    let mut twitter = Twitter::new();
    // User 1 posts a new tweet (id = 5)
    twitter.post_tweet(1, 5);
    // User 1's news feed should return [5]
    assert_eq!(twitter.get_news_feed(1), vec![5]);
    // User 1 follows user 2
    twitter.follow(1, 2);
    // User 2 posts a new tweet (id = 6)
    twitter.post_tweet(2, 6);
    // User 1's news feed should return [6, 5]
    assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
    // User 1 unfollows user 2
    twitter.unfollow(1, 2);
    // User 1's news feed should return [5]
    assert_eq!(twitter.get_news_feed(1), vec![5]);
}
