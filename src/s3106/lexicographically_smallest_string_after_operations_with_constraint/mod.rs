// Problem 3106: lexicographically smallest string after operations with constraint
// #Medium #String #Greedy #2024_04_11_Time_1_ms_(100.00%)_Space_42.4_MB_(91.10%)

pub struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut s_array: Vec<char> = s.chars().collect();
        let mut k = k;
        for i in 0..s_array.len() {
            let dist_to_a = Self::cyclic_distance(s_array[i], 'a');
            if dist_to_a <= k {
                s_array[i] = 'a';
                k -= dist_to_a;
            } else if k > 0 {
                s_array[i] = ((s_array[i] as u8) - (k as u8)) as char;
                k = 0;
            }
        }
        s_array.into_iter().collect()
    }

    fn cyclic_distance(ch1: char, ch2: char) -> i32 {
        let dist = (ch1 as i32 - ch2 as i32).abs();
        dist.min(26 - dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_smallest_string() {
        assert_eq!(Solution::get_smallest_string("zbbz".to_string(), 3), "aaaz");
    }

    #[test]
    fn test_get_smallest_string2() {
        assert_eq!(Solution::get_smallest_string("xaxcd".to_string(), 4), "aawcd");
    }

    #[test]
    fn test_get_smallest_string3() {
        assert_eq!(Solution::get_smallest_string("lol".to_string(), 0), "lol");
    }
}
