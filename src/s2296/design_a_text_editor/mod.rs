// Problem 2296: design a text editor

pub struct TextEditor {
    left: Vec<char>,
    right: Vec<char>,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn add_text(&mut self, text: String) {
        for c in text.chars() {
            self.left.push(c);
        }
    }

    pub fn delete_text(&mut self, k: i32) -> i32 {
        let mut deleted = 0;
        for _ in 0..k {
            if let Some(c) = self.left.pop() {
                deleted += 1;
            } else {
                break;
            }
        }
        deleted
    }

    pub fn cursor_left(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.left.pop() {
                self.right.push(c);
            } else {
                break;
            }
        }
        self.get_cursor_text()
    }

    pub fn cursor_right(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.right.pop() {
                self.left.push(c);
            } else {
                break;
            }
        }
        self.get_cursor_text()
    }

    fn get_cursor_text(&self) -> String {
        let n = self.left.len();
        let start = if n <= 10 { 0 } else { n - 10 };
        self.left[start..].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void textEditor()
    //   TextEditor textEditor = new TextEditor(); // The current text is "|". (The '|' character represents the cursor)
    //   textEditor.addText("leetcode"); // The current text is "leetcode|"
    //   assertThat(textEditor.deleteText(4), equalTo(4)); // The current text is "leet|"
    //   // 4 characters were deleted.
    //   // The current text is "leetpractice|"
    //   textEditor.addText("practice");
    //   assertThat(textEditor.cursorRight(3), equalTo("etpractice")); // The current text is "leetpractice|"
    //   // "etpractice" is the last min(10, 3) = 3 characters to the left of the cursor
    //   assertThat(textEditor.cursorLeft(8), equalTo("leet"));
    //   // The current text is "leet|practice"
    //   // "leet" is the last min(10, 4) = 4 characters to the left of the cursor
    //   assertThat(textEditor.deleteText(10), equalTo(4));
    //   // The current text is "|practice".
    //   // Only 4 characters were deleted.
    //   assertThat(textEditor.cursorLeft(2), equalTo(""));
    //   // The current text is "|practice"
    //   // The cursor cannot be moved beyond the actual text and thus did not move.
    //   // "" is the last min(10, 0) = 0 characters to the left of the cursor
    //   assertThat(textEditor.cursorRight(6), equalTo("practi")); // The current text is "practi|ce"
    //   // "practi" is the last min(10, 6) = 6 characters to the left of the cursor
    //   assertThat(textEditor.deleteText(4), equalTo(0));
    // }
    #[test]
    fn test_text_editor() {
        let mut text_editor = TextEditor::new();
        // The current text is "|"
        text_editor.add_text(String::from("leetcode"));
        // The current text is "leetcode|"
        assert_eq!(text_editor.delete_text(4), 4);
        // The current text is "leet|"
        text_editor.add_text(String::from("practice"));
        // The current text is "leetpractice|"
        assert_eq!(text_editor.cursor_right(3), String::from("etpractice"));
        // The current text is "leetpractice|"
        assert_eq!(text_editor.cursor_left(8), String::from("leet"));
        // The current text is "leet|practice"
        assert_eq!(text_editor.delete_text(10), 4);
        // The current text is "|practice"
        assert_eq!(text_editor.cursor_left(2), String::from(""));
        // The current text is "|practice"
        assert_eq!(text_editor.cursor_right(6), String::from("practi"));
        // The current text is "practi|ce"
        assert_eq!(text_editor.delete_text(4), 0);
    }
}
