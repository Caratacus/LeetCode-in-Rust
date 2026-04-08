// Problem 3085: minimum deletions to make string k special
// #Medium #String #Hash_Table #Sorting #Greedy #Counting

pub struct Solution;

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut arr = [0i32; 26];
        for c in word.chars() {
            arr[(c as usize) - ('a' as usize)] += 1;
        }

        let mut min = i32::MAX;
        for &value in &arr {
            if value != 0 {
                let u = value + k;
                let mut res = 0;
                for &i in &arr {
                    if i < value {
                        res += i;
                    } else if i > u {
                        res += i - u;
                    }
                }
                min = min.min(res);
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumDeletions()
    //   assertThat(new Solution().minimumDeletions("aabcaba", 0), equalTo(3));
    #[test]
    fn test_minimum_deletions() {
        assert_eq!(Solution::minimum_deletions("aabcaba".to_string(), 0), 3);
    }

    // Java: void minimumDeletions2()
    //   assertThat(new Solution().minimumDeletions("dabdcbdcdcd", 2), equalTo(2));
    #[test]
    fn test_minimum_deletions2() {
        assert_eq!(Solution::minimum_deletions("dabdcbdcdcd".to_string(), 2), 2);
    }

    // Java: void minimumDeletions3()
    //   assertThat(new Solution().minimumDeletions("aaabaaa", 2), equalTo(1));
    #[test]
    fn test_minimum_deletions3() {
        assert_eq!(Solution::minimum_deletions("aaabaaa".to_string(), 2), 1);
    }
}
