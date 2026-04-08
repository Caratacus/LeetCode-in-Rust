// Problem 3114: latest time you can obtain after replacing characters
// #Easy #String #Enumeration #2024_04_27_Time_1_ms_(100.00%)_Space_42.5_MB_(85.42%)

pub struct Solution;

impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut nm = String::new();

        // Handle hours (first two characters)
        if chars[0] == '?' && chars[1] == '?' {
            nm.push_str("11");
        } else if chars[0] != '?' && chars[1] == '?' {
            nm.push(chars[0]);
            if chars[0] == '1' {
                nm.push('1');
            } else {
                nm.push('9');
            }
        } else if chars[0] == '?' && chars[1] != '?' {
            if chars[1] >= '2' && chars[1] <= '9' {
                nm.push('0');
            } else {
                nm.push('1');
            }
            nm.push(chars[1]);
        } else {
            nm.push(chars[0]);
            nm.push(chars[1]);
        }

        nm.push(':');

        // Handle minutes (last two characters)
        if chars[3] == '?' && chars[4] == '?' {
            nm.push_str("59");
        } else if chars[3] != '?' && chars[4] == '?' {
            nm.push(chars[3]);
            nm.push('9');
        } else if chars[3] == '?' && chars[4] != '?' {
            nm.push('5');
            nm.push(chars[4]);
        } else {
            nm.push(chars[3]);
            nm.push(chars[4]);
        }

        nm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_latest_time() {
        assert_eq!(Solution::find_latest_time("1?:?4".to_string()), "11:54");
    }

    #[test]
    fn test_find_latest_time2() {
        assert_eq!(Solution::find_latest_time("0?:5?".to_string()), "09:59");
    }

    #[test]
    fn test_find_latest_time3() {
        assert_eq!(Solution::find_latest_time("?1:?6".to_string()), "11:56");
    }

    #[test]
    fn test_find_latest_time4() {
        assert_eq!(Solution::find_latest_time("08:33".to_string()), "08:33");
    }

    #[test]
    fn test_find_latest_time5() {
        assert_eq!(Solution::find_latest_time("??:1?".to_string()), "11:19");
    }

    #[test]
    fn test_find_latest_time6() {
        assert_eq!(Solution::find_latest_time("04:??".to_string()), "04:59");
    }

    #[test]
    fn test_find_latest_time7() {
        assert_eq!(Solution::find_latest_time("?3:12".to_string()), "03:12");
    }
}
