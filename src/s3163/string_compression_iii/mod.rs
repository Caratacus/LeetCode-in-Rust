// Problem 3163: string compression iii
// #Medium #String #2024_06_02_Time_17_ms_(88.10%)_Space_45.7_MB_(71.08%)

pub struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let chars: Vec<char> = word.chars().collect();
        let mut result = String::new();
        let mut last = chars[0];
        let mut count = 1;
        for i in 1..chars.len() {
            if chars[i] == last {
                count += 1;
                if count == 10 {
                    result.push('9');
                    result.push(last);
                    count = 1;
                }
            } else {
                result.push_str(&count.to_string());
                result.push(last);
                last = chars[i];
                count = 1;
            }
        }
        result.push_str(&count.to_string());
        result.push(last);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void compressedString()
    //   assertThat(new Solution().compressedString("abcde"), equalTo("1a1b1c1d1e"));
    #[test]
    fn test_compressed_string() {
        assert_eq!(
            Solution::compressed_string("abcde".to_string()),
            "1a1b1c1d1e"
        );
    }

    // Java: void compressedString2()
    //   assertThat(new Solution().compressedString("aaaaaaaaaaaaaabb"), equalTo("9a5a2b"));
    #[test]
    fn test_compressed_string2() {
        assert_eq!(
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_string()),
            "9a5a2b"
        );
    }
}
