// Problem 1585: Check If String Is Transformable With Substring Sort Operations
// #Hard #String #Sorting #Greedy
// #Big_O_Time_O(n)_Space_O(n)

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let n = s.len();
        if n != t.len() {
            return false;
        }

        // Store positions of each digit in s
        let mut pos: Vec<VecDeque<usize>> = vec![VecDeque::new(); 10];
        let s_bytes = s.as_bytes();
        for i in 0..n {
            pos[(s_bytes[i] - b'0') as usize].push_back(i);
        }

        let t_bytes = t.as_bytes();
        for i in 0..n {
            let digit = (t_bytes[i] - b'0') as usize;

            if pos[digit].is_empty() {
                return false;
            }

            // For each digit in t, we need to ensure we can bring it to position i
            // by swapping with smaller digits only
            for d in 0..digit {
                if !pos[d].is_empty() && *pos[d].front().unwrap() < *pos[digit].front().unwrap() {
                    return false;
                }
            }

            pos[digit].pop_front();
        }
        true
    }
}
