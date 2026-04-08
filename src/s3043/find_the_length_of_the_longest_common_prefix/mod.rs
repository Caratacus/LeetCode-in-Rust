// Problem 3043: Find the Length of the Longest Common Prefix
// #Medium #Array #String #Hash_Table #Trie
// #Big_O_Time_O(n*log(m))_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for num in &arr2 {
            trie.add_word(num.to_string());
        }

        let mut longest = 0;
        for num in &arr1 {
            let val = num.to_string();
            if val.len() > longest as usize {
                longest = longest.max(trie.find_longest_prefix(&val));
            }
        }
        longest
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            let code_point = (ch as usize) - ('0' as usize);
            if current.nodes[code_point].is_none() {
                current.nodes[code_point] = Some(Box::new(TrieNode::new()));
            }
            current = current.nodes[code_point].as_mut().unwrap();
        }
    }

    fn find_longest_prefix(&self, word: &str) -> i32 {
        let mut current = &self.root;
        let mut i = 0;
        for ch in word.chars() {
            let code_point = (ch as usize) - ('0' as usize);
            if current.nodes[code_point].is_none() {
                return i;
            }
            current = current.nodes[code_point].as_ref().unwrap();
            i += 1;
        }
        i
    }
}

struct TrieNode {
    nodes: [Option<Box<TrieNode>>; 10],
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            nodes: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000]),
            3
        );
    }

    #[test]
    fn test_longest_common_prefix2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]),
            0
        );
    }
}
