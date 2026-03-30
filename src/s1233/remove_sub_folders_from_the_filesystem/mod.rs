// Problem 1233: remove sub folders from the filesystem

pub struct Solution;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void removeSubfolders()
    //   assertThat(
    //   new Solution()
    //   .removeSubfolders(new String[] {"/a", "/a/b", "/c/d", "/c/d/e", "/c/f"}),
    //   equalTo(Arrays.asList("/a", "/c/d", "/c/f")));
    #[test]
    fn test_remove_subfolders() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeSubfolders2()
    //   assertThat(
    //   new Solution().removeSubfolders(new String[] {"/a", "/a/b/c", "/a/b/d"}),
    //   equalTo(Collections.singletonList("/a")));
    #[test]
    fn test_remove_subfolders2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void removeSubfolders3()
    //   assertThat(
    //   new Solution().removeSubfolders(new String[] {"/a/b/c", "/a/b/ca", "/a/b/d"}),
    //   equalTo(Arrays.asList("/a/b/c", "/a/b/ca", "/a/b/d")));
    #[test]
    fn test_remove_subfolders3() {
        // TODO: 翻译 Java 测试
    }
}
