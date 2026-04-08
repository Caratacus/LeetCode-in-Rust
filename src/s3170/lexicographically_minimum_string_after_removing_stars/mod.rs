// Problem 3170: lexicographically minimum string after removing stars
// #Medium #String #Hash_Table #Greedy #Stack #Heap_Priority_Queue
// #2024_06_06_Time_29_ms_(99.93%)_Space_45.6_MB_(92.80%)

pub struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut arr: Vec<char> = s.chars().collect();
        let mut idx_chain: Vec<i32> = vec![-1; arr.len()];
        let mut last_idx: Vec<i32> = vec![-1; 26];
        for i in 0..arr.len() {
            if arr[i] == '*' {
                for j in 0..26 {
                    if last_idx[j] != -1 {
                        arr[last_idx[j] as usize] = '#';
                        last_idx[j] = idx_chain[last_idx[j] as usize];
                        break;
                    }
                }
                arr[i] = '#';
            } else {
                idx_chain[i] = last_idx[(arr[i] as usize) - ('a' as usize)];
                last_idx[(arr[i] as usize) - ('a' as usize)] = i as i32;
            }
        }
        let mut result = String::new();
        for c in arr {
            if c != '#' {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void clearStars()
    //   assertThat(new Solution().clearStars("aaba*"), equalTo("aab"));
    #[test]
    fn test_clear_stars() {
        assert_eq!(Solution::clear_stars("aaba*".to_string()), "aab");
    }

    // Java: void clearStars2()
    //   assertThat(new Solution().clearStars("abc"), equalTo("abc"));
    #[test]
    fn test_clear_stars2() {
        assert_eq!(Solution::clear_stars("abc".to_string()), "abc");
    }
}
