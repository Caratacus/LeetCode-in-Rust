// Problem 2296: design a text editor

pub struct TextEditor {}

impl TextEditor {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_text(&mut self, text: String) -> () {
        todo!()
    }

    pub fn delete_text(&mut self, k: i32) -> i32 {
        todo!()
    }

    pub fn cursor_left(&mut self, k: i32) -> String {
        todo!()
    }

    pub fn cursor_right(&mut self, k: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void textEditor()
    //   // The current text is "|". (The '|' character represents the cursor)
    //   TextEditor textEditor = new TextEditor();
    //   // The current text is "leetcode|".
    //   textEditor.addText("leetcode");
    //   assertThat(textEditor.deleteText(4), equalTo(4));
    //   ... (21 more lines)
    #[test]
    fn test_text_editor() {
        // TODO: 翻译 Java 测试
    }
}
