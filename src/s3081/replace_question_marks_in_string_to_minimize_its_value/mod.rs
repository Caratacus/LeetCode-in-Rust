// Problem 3081: replace question marks in string to minimize its value
// #Medium #String #Hash_Table #Sorting #Greedy #Heap_Priority_Queue #Counting

pub struct Solution;

impl Solution {
    pub fn minimize_string_value(s: String) -> String {
        let n = s.len();
        let mut time = 0;
        let mut count = [0i32; 26];
        let mut res = [0i32; 26];
        let mut str: Vec<char> = s.chars().collect();

        for c in &str {
            if *c != '?' {
                count[(*c as usize) - ('a' as usize)] += 1;
            } else {
                time += 1;
            }
        }

        let mut min_time = *count.iter().min().unwrap();

        while time > 0 {
            for j in 0..26 {
                if count[j] == min_time {
                    res[j] += 1;
                    count[j] += 1;
                    time -= 1;
                }
                if time == 0 {
                    break;
                }
            }
            min_time += 1;
        }

        let mut start = 0;
        for i in 0..n {
            if str[i] == '?' {
                while res[start] == 0 {
                    start += 1;
                }
                str[i] = (b'a' + start as u8) as char;
                res[start] -= 1;
            }
        }

        str.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimizeStringValue()
    //   assertThat(new Solution().minimizeStringValue("???"), equalTo("abc"));
    #[test]
    fn test_minimize_string_value() {
        assert_eq!(Solution::minimize_string_value("???".to_string()), "abc");
    }

    // Java: void minimizeStringValue2()
    //   assertThat(new Solution().minimizeStringValue("a?a?"), equalTo("abac"));
    #[test]
    fn test_minimize_string_value2() {
        assert_eq!(Solution::minimize_string_value("a?a?".to_string()), "abac");
    }
}
