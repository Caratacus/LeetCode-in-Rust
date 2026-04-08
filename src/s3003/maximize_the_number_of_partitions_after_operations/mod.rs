// Problem 3003: maximize the number of partitions after operations
// #Hard #String #Dynamic_Programming #Bit_Manipulation #Bitmask
// #2024_02_26_Time_1_ms_(100.00%)_Space_42.1_MB_(99.44%)

pub struct Solution;

impl Solution {
    const ALPHABET_SIZE: i32 = ('z' as i32) - ('a' as i32) + 1;

    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        if k == Self::ALPHABET_SIZE {
            return 1;
        }
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut ansr = vec![0i32; n];
        let mut usedr = vec![0i32; n];
        let mut used = 0i32;
        let mut cnt_used = 0i32;
        let mut ans = 1i32;
        for i in (0..n).rev() {
            let ch = (s_bytes[i] - b'a') as i32;
            if used & (1 << ch) == 0 {
                if cnt_used == k {
                    cnt_used = 0;
                    used = 0;
                    ans += 1;
                }
                used |= 1 << ch;
                cnt_used += 1;
            }
            ansr[i] = ans;
            usedr[i] = used;
        }
        let mut ansl = 0i32;
        ans = ansr[0];
        let mut l = 0usize;
        while l < n {
            used = 0;
            cnt_used = 0;
            let mut used_before_last = 0i32;
            let mut used_twice_before_last = 0i32;
            let mut last = -1i32;
            let mut r = l;
            while r < n {
                let ch = (s_bytes[r] - b'a') as i32;
                if used & (1 << ch) == 0 {
                    if cnt_used == k {
                        break;
                    }
                    used_before_last = used;
                    last = r as i32;
                    used |= 1 << ch;
                    cnt_used += 1;
                } else if cnt_used < k {
                    used_twice_before_last |= 1 << ch;
                }
                r += 1;
            }
            if cnt_used == k {
                if last - l as i32 > (used_before_last.count_ones() as i32) {
                    ans = std::cmp::max(ans, ansl + 1 + ansr[last as usize]);
                }
                if last + 1 < r as i32 {
                    if last + 2 >= n as i32 {
                        ans = std::cmp::max(ans, ansl + 1 + 1);
                    } else {
                        if (usedr[last as usize + 2].count_ones() as i32) == k {
                            let can_use = ((1 << Self::ALPHABET_SIZE) - 1) & !used & !usedr[last as usize + 2];
                            if can_use > 0 {
                                ans = std::cmp::max(ans, ansl + 1 + 1 + ansr[last as usize + 2]);
                            } else {
                                ans = std::cmp::max(ans, ansl + 1 + ansr[last as usize + 2]);
                            }
                            let l1 = (s_bytes[last as usize + 1] - b'a') as i32;
                            if used_twice_before_last & (1 << l1) == 0 {
                                ans = std::cmp::max(ans, ansl + 1 + ansr[last as usize + 1]);
                            }
                        } else {
                            ans = std::cmp::max(ans, ansl + 1 + ansr[last as usize + 2]);
                        }
                    }
                }
            }
            l = r;
            ansl += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_partitions_after_operations() {
        assert_eq!(Solution::max_partitions_after_operations("accca".to_string(), 2), 3);
    }

    #[test]
    fn test_max_partitions_after_operations2() {
        assert_eq!(Solution::max_partitions_after_operations("aabaab".to_string(), 3), 1);
    }

    #[test]
    fn test_max_partitions_after_operations3() {
        assert_eq!(Solution::max_partitions_after_operations("xxyz".to_string(), 1), 4);
    }
}
