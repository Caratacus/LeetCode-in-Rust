// Problem 1593: Split a String Into the Max Number of Unique Substrings
// #Medium #String #Hash_Table #Backtracking
// #Big_O_Time_O(2^n)_Space_O(n)

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s.len();
        let mut lo = 1;
        let mut hi = n;

        // Binary search
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if Self::ok(0, mid, 0, s_bytes, &mut HashSet::new()) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo as i32
    }

    fn ok(
        depth: usize,
        end: usize,
        cur_len: usize,
        s: &[u8],
        seen: &mut HashSet<Vec<u8>>,
    ) -> bool {
        if depth == end {
            return true;
        }

        for j in cur_len..s.len() {
            // Not enough length remains to reach the end
            if s.len() - j < end - depth {
                break;
            }

            let cur = s[cur_len..=j].to_vec();
            if seen.insert(cur) {
                if Self::ok(depth + 1, end, j + 1, s, seen) {
                    return true;
                }
                seen.remove(&s[cur_len..=j].to_vec());
            }
        }
        false
    }
}
