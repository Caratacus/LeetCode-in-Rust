// Problem 0355: design twitter

pub struct Twitter {}

impl Twitter {
    pub fn new() -> Self {
        todo!()
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) -> () {
        todo!()
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        todo!()
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) -> () {
        todo!()
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) -> () {
        todo!()
    }

    pub fn check_new_user(&mut self, user_id: i32) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void twitter()
    //   Twitter twitter = new Twitter();
    //   // User 1 posts a new tweet (id = 5).
    //   twitter.postTweet(1, 5);
    //   // User 1's news feed should return a list with 1 tweet id -> [5]. return [5]
    //   assertThat(twitter.getNewsFeed(1), equalTo(Arrays.asList(5)));
    //   ... (11 more lines)
    #[test]
    fn test_twitter() {
        // TODO: 翻译 Java 测试
    }
}
