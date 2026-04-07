// Tests for Problem 2296: Design a Text Editor
// Java reference: src/test/java/g2201_2300/s2296_design_a_text_editor/TextEditorTest.java

use leetcode_in_rust::s2296::design_a_text_editor::TextEditor;

#[test]
fn test_text_editor() {
    // The current text is "|". (The '|' character represents the cursor)
    let mut text_editor = TextEditor::new();
    // The current text is "leetcode|".
    text_editor.add_text(String::from("leetcode"));
    assert_eq!(text_editor.delete_text(4), 4);
    // The current text is "leet|".
    // 4 characters were deleted.
    // The current text is "leetpractice|".
    text_editor.add_text(String::from("practice"));
    assert_eq!(text_editor.cursor_right(3), String::from("etpractice"));
    // The current text is "leetpractice|".
    // The cursor cannot be moved beyond the actual text and thus did not move.
    // "etpractice" is the last 10 characters to the left of the cursor.
    assert_eq!(text_editor.cursor_left(8), String::from("leet"));
    // The current text is "leet|practice".
    // "leet" is the last min(10, 4) = 4 characters to the left of the cursor.
    assert_eq!(text_editor.delete_text(10), 4);
    // The current text is "|practice".
    // Only 4 characters were deleted.
    assert_eq!(text_editor.cursor_left(2), String::from(""));
    // The current text is "|practice".
    // The cursor cannot be moved beyond the actual text and thus did not move.
    // "" is the last min(10, 0) = 0 characters to the left of the cursor.
    assert_eq!(text_editor.cursor_right(6), String::from("practi"));
    // The current text is "practi|ce".
    // "practi" is the last min(10, 6) = 6 characters to the left of the cursor.
}
