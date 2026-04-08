// Problem 3093: longest common suffix queries
// #Hard #Array #String #Trie #2024_04_18_Time_39_ms_(93.67%)_Space_160.9_MB_(66.40%)

pub struct Solution;

struct Trie {
    ch: [Option<Box<Trie>>; 26],
    index: usize,
}

impl Trie {
    fn new(index: usize) -> Self {
        Trie {
            ch: Default::default(),
            index,
        }
    }

    fn get(&self, ch: char) -> Option<&Trie> {
        self.ch[(ch as usize) - ('a' as usize)].as_deref()
    }

    fn has(&self, ch: char) -> bool {
        self.ch[(ch as usize) - ('a' as usize)].is_some()
    }

    fn put(&mut self, ch: char, index: usize) {
        self.ch[(ch as usize) - ('a' as usize)] = Some(Box::new(Trie::new(index)));
    }

    fn get_mut(&mut self, ch: char) -> &mut Trie {
        self.ch[(ch as usize) - ('a' as usize)].as_mut().unwrap()
    }
}

impl Solution {
    pub fn string_indices(wc: Vec<String>, wq: Vec<String>) -> Vec<i32> {
        let mut min_length = wc[0].len();
        let mut min_index = 0;
        let n = wc.len();
        let m = wq.len();

        for i in 0..n {
            if min_length > wc[i].len() {
                min_length = wc[i].len();
                min_index = i;
            }
        }

        let mut root = Trie::new(min_index);

        for i in 0..n {
            let mut curr = &mut root;
            for j in (0..wc[i].len()).rev() {
                let ch = wc[i].chars().nth(j).unwrap();
                if curr.has(ch) {
                    let next = curr.get_mut(ch);
                    if wc[next.index].len() > wc[i].len() {
                        next.index = i;
                    }
                    curr = next;
                } else {
                    curr.put(ch, i);
                    curr = curr.get_mut(ch);
                }
            }
        }

        let mut ans: Vec<i32> = vec![0; m];
        for i in 0..m {
            let mut curr = &root;
            for j in (0..wq[i].len()).rev() {
                let ch = wq[i].chars().nth(j).unwrap();
                if curr.has(ch) {
                    curr = curr.get(ch).unwrap();
                } else {
                    break;
                }
            }
            ans[i] = curr.index as i32;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void stringIndices()
    //   assertThat(
    //   new Solution()
    //   .stringIndices(
    //   new String[] {"abcd", "bcd", "xbcd"},
    //   new String[] {"cd", "bcd", "xyz"}),
    //   equalTo(new int[] {1, 1, 1}));
    #[test]
    fn test_string_indices() {
        assert_eq!(
            Solution::string_indices(
                vec!["abcd".to_string(), "bcd".to_string(), "xbcd".to_string()],
                vec!["cd".to_string(), "bcd".to_string(), "xyz".to_string()]
            ),
            vec![1, 1, 1]
        );
    }

    // Java: void stringIndices2()
    //   assertThat(
    //   new Solution()
    //   .stringIndices(
    //   new String[] {"abcdefgh", "poiuygh", "ghghgh"},
    //   new String[] {"gh", "acbfgh", "acbfegh"}),
    //   equalTo(new int[] {2, 0, 2}));
    #[test]
    fn test_string_indices2() {
        assert_eq!(
            Solution::string_indices(
                vec![
                    "abcdefgh".to_string(),
                    "poiuygh".to_string(),
                    "ghghgh".to_string()
                ],
                vec!["gh".to_string(), "acbfgh".to_string(), "acbfegh".to_string()]
            ),
            vec![2, 0, 2]
        );
    }
}
