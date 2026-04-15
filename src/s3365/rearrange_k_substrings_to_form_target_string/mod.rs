// Problem 3365: Rearrange K Substrings to Form Target String
// #Medium #String #Hash_Table #Sorting

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let size = s.len();
        let div = size / k as usize;
        let mut map: HashMap<&str, i32> = HashMap::new();
        for i in (0..size).step_by(div) {
            let sub = &s[i..i + div];
            *map.entry(sub).or_insert(0) += 1;
        }
        for i in (0..size).step_by(div) {
            let sub = &t[i..i + div];
            if let Some(count) = map.get_mut(sub) {
                if *count > 0 {
                    *count -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isPossibleToRearrange()
    //   assertThat(new Solution().isPossibleToRearrange("abcd", "cdab", 2), equalTo(true));
    #[test]
    fn test_is_possible_to_rearrange() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isPossibleToRearrange2()
    //   assertThat(new Solution().isPossibleToRearrange("aabbcc", "bbaacc", 3), equalTo(true));
    #[test]
    fn test_is_possible_to_rearrange2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void isPossibleToRearrange3()
    //   assertThat(new Solution().isPossibleToRearrange("aabbcc", "bbaacc", 2), equalTo(false));
    #[test]
    fn test_is_possible_to_rearrange3() {
        // TODO: 翻译 Java 测试
    }
}
