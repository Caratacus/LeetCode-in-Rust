// Problem 2301: match substring after replacement

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let mut map: HashMap<char, HashSet<char>> = HashMap::new();
        for m in mappings {
            let entry = map.entry(m[0]).or_default();
            entry.insert(m[1]);
        }

        let s_chars: Vec<char> = s.chars().collect();
        let sub_chars: Vec<char> = sub.chars().collect();
        let n = s.len();
        let m = sub.len();

        if m > n {
            return false;
        }

        for i in 0..=(n - m + 1) {
            let mut is_match = true;
            for j in 0..m {
                let sc = s_chars[i + j];
                let pc = sub_chars[j];
                if sc != pc {
                    // Check if sc is a valid replacement for pc
                    if let Some(valid_replacements) = map.get(&pc) {
                        if !valid_replacements.contains(&sc) {
                            is_match = false;
                            break;
                        }
                    } else {
                        is_match = false;
                        break;
                    }
                }
            }
            if is_match {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void matchReplacement()
    //   assertThat(
    //   new Solution()
    //   .matchReplacement(
    //   "fool3e7bar",
    //   "leet",
    //   new char[][] {{'e', '3'}, {'t', '7'}, {'l', '1'}, {'o', '0'}}),
    //   equalTo(true));
    #[test]
    fn test_match_replacement() {
        assert_eq!(
            Solution::match_replacement(
                String::from("fool3e7bar"),
                String::from("leet"),
                vec
![vec
!['e', '3'], vec
!['t', '7'], vec
!['l', '1'], vec
!['o', '0']]
            ),
            true
        );
    }

    // Java: void matchReplacement2()
    //   assertThat(
    //   new Solution().matchReplacement("fooleetbar", "f00l", new char[][] {{'o', '0'}}),
    //   equalTo(false));
    #[test]
    fn test_match_replacement2() {
        assert_eq!(
            Solution::match_replacement(
                String::from("fooleetbar"),
                String::from("f00l"),
                vec
![vec
!['o', '0']]
            ),
            false
        );
    }

    // Java: void matchReplacement3()
    //   assertThat(
    //   new Solution()
    //   .matchReplacement(
    //   "Fool33tbaR",
    //   "leetd",
    //   new char[][] {{'e', '3'}, {'t', '7'}, {'l', '1'}, {'o', '0'}, {'d', 'b'}}),
    //   equalTo(false));
    #[test]
    fn test_match_replacement3() {
        assert_eq!(
            Solution::match_replacement(
                String::from("Fool33tbaR"),
                String::from("leetd"),
                vec
![vec
!['e', '3'], vec
!['t', '7'], vec
!['l', '1'], vec
!['o', '0'], vec
!['d', 'b']]
            ),
            false
        );
    }
}
