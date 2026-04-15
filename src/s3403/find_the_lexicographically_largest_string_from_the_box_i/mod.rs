// Problem 3403: find the lexicographically largest string from the box i
// #Medium #String #Two_Pointers #Enumeration

pub struct Solution;

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let n = word.len();
        let maxlen = n - num_friends as usize + 1;
        let bytes = word.as_bytes();
        let mut max_char = bytes[0];
        let mut res = String::new();
        for i in 0..n {
            if bytes[i] >= max_char {
                let end = (i + maxlen).min(n);
                let curr = &word[i..end];
                if curr > res.as_str() {
                    res = curr.to_string();
                }
                max_char = bytes[i];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void answerString()
    //   assertThat(new Solution().answerString("dbca", 2), equalTo("dbc"));
    #[test]
    fn test_answer_string() {
        // TODO: 翻译 Java 测试
    }

    // Java: void answerString2()
    //   assertThat(new Solution().answerString("gggg", 4), equalTo("g"));
    #[test]
    fn test_answer_string2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void answerString3()
    //   assertThat(new Solution().answerString("a", 1), equalTo("a"));
    #[test]
    fn test_answer_string3() {
        // TODO: 翻译 Java 测试
    }
}
