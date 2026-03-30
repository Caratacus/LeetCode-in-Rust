// Problem 1472: design browser history

pub struct BrowserHistory {
    homepage: String,
}

impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        todo!()
    }

    pub fn visit(&mut self, url: String) -> () {
        todo!()
    }

    pub fn back(&mut self, steps: i32) -> String {
        todo!()
    }

    pub fn forward(&mut self, steps: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void browserHistoryTest()
    //   BrowserHistory browserHistory = new BrowserHistory("leetcode.com");
    //   browserHistory.visit("google.com");
    //   browserHistory.visit("facebook.com");
    //   browserHistory.visit("youtube.com");
    //   assertThat(browserHistory.back(1), equalTo("facebook.com"));
    //   ... (6 more lines)
    #[test]
    fn test_browser_history_test() {
        // TODO: 翻译 Java 测试
    }
}
