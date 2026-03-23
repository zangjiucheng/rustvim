//! # History Tests for Vimlike Editor
//!
//! This test suite verifies the correctness of the undo/redo history system for a Vim-like text editor.
//! It covers a wide range of editing scenarios, including:
//! - Basic undo/redo mechanics
//! - Clearing the redo stack on new actions
//! - Insertions and deletions involving newlines and multi-line edits
//! - Complex insert mode sessions and mode switches
//! - Real-world editing workflows with mixed operations
//! - Open line commands (`o` and `O`) and their undo/redo behavior
//! - Insert mode deletions (e.g., backspace) and their undo/redo tracking
//! - Mixed insert and delete operations, ensuring correct sequencing of history actions
//!
//! Each test simulates realistic editing actions and asserts that the buffer and history behave as expected.
//! The goal is to ensure robust and predictable undo/redo functionality for all supported editing commands.
//! Run with `cargo test --test history_tests` to execute the tests.

// Import the actual modules from the main crate
use rustvim::buffer;
use rustvim::buffer::Position;
use rustvim::history::{EditAction, History};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_undo_redo() {
        let mut history = History::new();
        let action = EditAction::insert_text(Position::new(0, 0), "hello".to_string());

        // Push action
        history.push(action.clone());
        assert!(history.can_undo());
        assert!(!history.can_redo());

        // Undo
        let undone = history.pop_undo().unwrap();
        assert!(!history.can_undo());
        assert!(!history.can_redo());

        // Push to redo stack
        history.push_redo(undone);
        assert!(!history.can_undo());
        assert!(history.can_redo());

        // Redo
        let _redone = history.pop_redo().unwrap();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_new_action_clears_redo() {
        let mut history = History::new();

        // Add and undo an action
        history.push(EditAction::insert_text(
            Position::new(0, 0),
            "hello".to_string(),
        ));
        let undone = history.pop_undo().unwrap();
        history.push_redo(undone);
        assert!(history.can_redo());

        // Add new action - should clear redo stack
        history.push(EditAction::insert_text(
            Position::new(0, 5),
            "world".to_string(),
        ));
        assert!(!history.can_redo());
        assert!(history.can_undo());
    }

    #[test]
    fn test_undo_insert_with_newlines() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Insert some initial text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');

        // Now simulate insert mode: cursor at end, insert "\nWorld"
        let start_pos = crate::buffer::Position::new(0, 5);
        let inserted_text = "\nWorld"; // newline + "World"

        // Do the actual insertion
        buffer.insert_newline(crate::buffer::Position::new(0, 5));
        buffer.insert_char(crate::buffer::Position::new(1, 0), 'W');
        buffer.insert_char(crate::buffer::Position::new(1, 1), 'o');
        buffer.insert_char(crate::buffer::Position::new(1, 2), 'r');
        buffer.insert_char(crate::buffer::Position::new(1, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(1, 4), 'd');

        // Buffer should now have:
        // Line 0: "Hello"
        // Line 1: "World"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");

        // Record the insert action
        let action = EditAction::insert_text(start_pos, inserted_text.to_string());
        history.push(action);

        // Now undo - this should remove the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());

        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
    }

    #[test]
    fn test_complex_insert_mode_switches() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Start with some text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');

        // First insert mode session: add " World"
        let action1 =
            EditAction::insert_text(crate::buffer::Position::new(0, 5), " World".to_string());
        // Simulate the insertion
        for (i, ch) in " World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, 5 + i), ch);
        }
        history.push(action1);

        // Buffer should now be: "Hello World"
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");

        // Second insert mode session: add newline and "Test"
        let action2 =
            EditAction::insert_text(crate::buffer::Position::new(0, 11), "\nTest".to_string());
        // Simulate the insertion
        buffer.insert_newline(crate::buffer::Position::new(0, 11));
        for (i, ch) in "Test".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }
        history.push(action2);

        // Buffer should now be:
        // Line 0: "Hello World"
        // Line 1: "Test"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");

        // Third insert mode session: insert at beginning of first line
        let action3 =
            EditAction::insert_text(crate::buffer::Position::new(0, 0), "Hi ".to_string());
        // Simulate the insertion (this shifts existing text right)
        for (i, ch) in "Hi ".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        history.push(action3);

        // Buffer should now be:
        // Line 0: "Hi Hello World"
        // Line 1: "Test"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hi Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");

        // Now test undo operations

        // First undo: should remove "Hi " from beginning
        let result1 = history.apply_undo(&mut buffer);
        assert!(result1.is_some());
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");

        // Second undo: should remove "\nTest"
        let result2 = history.apply_undo(&mut buffer);
        assert!(result2.is_some());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");

        // Third undo: should remove " World"
        let result3 = history.apply_undo(&mut buffer);
        assert!(result3.is_some());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Test redo operations

        // First redo: should add " World" back
        let redo1 = history.apply_redo(&mut buffer);
        assert!(redo1.is_some());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");

        // Second redo: should add "\nTest" back
        let redo2 = history.apply_redo(&mut buffer);
        assert!(redo2.is_some());
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");

        // Third redo: should add "Hi " back at beginning
        let redo3 = history.apply_redo(&mut buffer);
        assert!(redo3.is_some());
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hi Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");
    }

    #[test]
    fn test_real_world_mixed_operations() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Simulate a realistic editing session

        // 1. Start with empty buffer, type "fn main() {"
        let action1 = EditAction::insert_text(
            crate::buffer::Position::new(0, 0),
            "fn main() {".to_string(),
        );
        for (i, ch) in "fn main() {".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        history.push(action1);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");

        // 2. Hit Enter and add indented content
        let action2 = EditAction::insert_text(
            crate::buffer::Position::new(0, 11),
            "\n    println!(\"Hello, world!\");".to_string(),
        );
        buffer.insert_newline(crate::buffer::Position::new(0, 11));
        for (i, ch) in "    println!(\"Hello, world!\");".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }
        history.push(action2);
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(
            buffer.get_line(1).unwrap(),
            "    println!(\"Hello, world!\");"
        );

        // 3. Add closing brace on new line
        let action3 =
            EditAction::insert_text(crate::buffer::Position::new(1, 31), "\n}".to_string());
        buffer.insert_newline(crate::buffer::Position::new(1, 31));
        buffer.insert_char(crate::buffer::Position::new(2, 0), '}');
        history.push(action3);
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.get_line(2).unwrap(), "}");

        // 4. Go back and add a comment on first line
        let action4 = EditAction::insert_text(
            crate::buffer::Position::new(0, 0),
            "// Main function\n".to_string(),
        );
        buffer.insert_char(crate::buffer::Position::new(0, 0), '/');
        buffer.insert_char(crate::buffer::Position::new(0, 1), '/');
        buffer.insert_char(crate::buffer::Position::new(0, 2), ' ');
        for (i, ch) in "Main function".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, 3 + i), ch);
        }
        buffer.insert_newline(crate::buffer::Position::new(0, 16));
        history.push(action4);

        // Should now have 4 lines with the comment at top
        assert_eq!(buffer.line_count(), 4);
        assert_eq!(buffer.get_line(0).unwrap(), "// Main function");
        assert_eq!(buffer.get_line(1).unwrap(), "fn main() {");
        assert_eq!(
            buffer.get_line(2).unwrap(),
            "    println!(\"Hello, world!\");"
        );
        assert_eq!(buffer.get_line(3).unwrap(), "}");

        // Now test undoing the entire session step by step

        // Undo 1: Remove comment
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(
            buffer.get_line(1).unwrap(),
            "    println!(\"Hello, world!\");"
        );
        assert_eq!(buffer.get_line(2).unwrap(), "}");

        // Undo 2: Remove closing brace
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(
            buffer.get_line(1).unwrap(),
            "    println!(\"Hello, world!\");"
        );

        // Undo 3: Remove println line
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");

        // Undo 4: Remove function declaration
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "");

        // Now test redoing everything back

        // Redo 1: Add function declaration
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");

        // Redo 2: Add println line
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(
            buffer.get_line(1).unwrap(),
            "    println!(\"Hello, world!\");"
        );

        // Redo 3: Add closing brace
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(
            buffer.get_line(1).unwrap(),
            "    println!(\"Hello, world!\");"
        );
        assert_eq!(buffer.get_line(2).unwrap(), "}");

        // Redo 4: Add comment
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 4);
        assert_eq!(buffer.get_line(0).unwrap(), "// Main function");
        assert_eq!(buffer.get_line(1).unwrap(), "fn main() {");
        assert_eq!(
            buffer.get_line(2).unwrap(),
            "    println!(\"Hello, world!\");"
        );
        assert_eq!(buffer.get_line(3).unwrap(), "}");
    }

    #[test]
    fn test_open_line_commands_undo() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Start with some text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');

        // Buffer should be: "Hello"
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Simulate "o" command: opens line below and enters insert mode
        // This should create a new line and then track any text inserted
        let open_line_pos = crate::buffer::Position::new(0, 5); // end of current line
        buffer.insert_newline(open_line_pos);

        // Now cursor should be at (1, 0) and in insert mode
        // Simulate typing "World" in insert mode
        let full_inserted_text = "\nWorld"; // This should include the newline from "o"

        // Insert the text
        for (i, ch) in "World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }

        // Record the complete action (newline + text)
        let action = EditAction::insert_text(open_line_pos, full_inserted_text.to_string());
        history.push(action);

        // Buffer should now be:
        // Line 0: "Hello"
        // Line 1: "World"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");

        // Test undo - should remove both the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());

        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Test redo - should restore the newline and "World"
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some());

        // After redo, should have both lines again
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");
    }

    #[test]
    fn test_open_line_above_command_undo() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Start with some text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');

        // Buffer should be: "Hello"
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Simulate "O" command: opens line above current line and enters insert mode
        // This should create a new line above and then track any text inserted
        let open_line_pos = crate::buffer::Position::new(0, 0); // beginning of current line
        buffer.insert_newline(open_line_pos);

        // After insert_newline(0,0), the buffer becomes:
        // Line 0: "" (empty line)
        // Line 1: "Hello" (original content pushed down)

        // Now cursor should be at (0, 0) and in insert mode
        // Simulate typing "World" in insert mode
        let full_inserted_text = "\nWorld"; // This should include the newline from "O"

        // Insert the text on the new empty line (line 0)
        for (i, ch) in "World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }

        // Record the complete action (newline + text)
        let action = EditAction::insert_text(open_line_pos, full_inserted_text.to_string());
        history.push(action);

        // Buffer should now be:
        // Line 0: "World"
        // Line 1: "Hello"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "World");
        assert_eq!(buffer.get_line(1).unwrap(), "Hello");

        // Test undo - should remove both the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());

        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Test redo - should restore the newline and "World"
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some());

        // After redo, should have both lines again
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "World");
        assert_eq!(buffer.get_line(1).unwrap(), "Hello");
    }

    #[test]
    fn test_open_line_below_command_undo() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Start with one line: "Hello"
        for (i, ch) in "Hello".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Simulate "o" command at the end of line 0
        // The "o" command should create a new line below and enter insert mode
        let open_line_pos = crate::buffer::Position::new(0, 5); // End of "Hello"

        // Insert a newline at the end of the current line
        buffer.insert_newline(open_line_pos);

        // Now buffer should be:
        // Line 0: "Hello"
        // Line 1: ""     (empty new line)
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "");

        // Insert the text on the new empty line (line 1)
        for (i, ch) in "World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }

        // Buffer should now be:
        // Line 0: "Hello"
        // Line 1: "World"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");

        // For "o" command, we record it differently - as regular insertion starting from where newline was inserted
        // The insertion includes newline + text, but positioned at end of original line
        let full_inserted_text = "\nWorld"; // newline + the text typed
        let action = EditAction::insert_text(open_line_pos, full_inserted_text.to_string());
        history.push(action);

        // Test undo - should remove both the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());

        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Test redo - should restore the newline and "World"
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some());

        // After redo, should have both lines again
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");
    }

    #[test]
    fn test_insert_mode_backspace_only_creates_undo_action() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Set up initial content: "Hello World"
        for (i, ch) in "Hello World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");

        // Simulate the scenario: Enter insert mode and only use backspace to delete existing content
        // This tests that deletions during insert mode are properly tracked for undo

        // Record a delete action as if insert mode tracked the deletion of " World"
        let deleted_text = " World";
        let deletion_pos = crate::buffer::Position::new(0, 5); // Position where " World" starts

        // Manually delete the text (simulating what would happen during insert mode)
        for _ in 0..deleted_text.len() {
            let delete_pos = crate::buffer::Position::new(0, buffer.line_length(0) - 1);
            buffer.delete_char(delete_pos);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Record the deletion action (this is what the fixed insert mode should do)
        let action = EditAction::delete_text(deletion_pos, deleted_text.to_string());
        history.push(action);

        // Test undo - should restore the deleted text
        let undo_result = history.apply_undo(&mut buffer);
        assert!(
            undo_result.is_some(),
            "Undo should be available for insert mode deletions"
        );
        assert_eq!(
            buffer.get_line(0).unwrap(),
            "Hello World",
            "Undo should restore deleted text"
        );

        // Test redo - should delete the text again
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some(), "Redo should be available");
        assert_eq!(
            buffer.get_line(0).unwrap(),
            "Hello",
            "Redo should delete text again"
        );
    }

    #[test]
    fn test_mixed_insert_and_delete_operations() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();

        // Set up initial content: "Hello"
        for (i, ch) in "Hello".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Test Case 1: Insert some text, then delete some existing content

        // 1. Insert " World" at the end
        let insert_pos = crate::buffer::Position::new(0, 5);
        let inserted_text = " World";
        for (i, ch) in inserted_text.chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, 5 + i), ch);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");

        // Record the insertion
        let insert_action = EditAction::insert_text(insert_pos, inserted_text.to_string());
        history.push(insert_action);

        // 2. Delete " World" (simulating backspace in insert mode or 'x' command)
        let delete_pos = crate::buffer::Position::new(0, 5);
        let deleted_text = " World";
        for _ in 0..deleted_text.len() {
            buffer.delete_char(crate::buffer::Position::new(0, 5));
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");

        // Record the deletion
        let delete_action = EditAction::delete_text(delete_pos, deleted_text.to_string());
        history.push(delete_action);

        // Test undo sequence

        // First undo: should restore " World"
        let undo1 = history.apply_undo(&mut buffer);
        assert!(undo1.is_some(), "First undo should work");
        assert_eq!(
            buffer.get_line(0).unwrap(),
            "Hello World",
            "First undo should restore deleted text"
        );

        // Second undo: should remove " World" again (undoing the insert)
        let undo2 = history.apply_undo(&mut buffer);
        assert!(undo2.is_some(), "Second undo should work");
        assert_eq!(
            buffer.get_line(0).unwrap(),
            "Hello",
            "Second undo should remove inserted text"
        );

        // Test redo sequence

        // First redo: should add " World" back
        let redo1 = history.apply_redo(&mut buffer);
        assert!(redo1.is_some(), "First redo should work");
        assert_eq!(
            buffer.get_line(0).unwrap(),
            "Hello World",
            "First redo should restore inserted text"
        );

        // Second redo: should delete " World" again
        let redo2 = history.apply_redo(&mut buffer);
        assert!(redo2.is_some(), "Second redo should work");
        assert_eq!(
            buffer.get_line(0).unwrap(),
            "Hello",
            "Second redo should delete text again"
        );

        // Verify we can undo again
        let undo3 = history.apply_undo(&mut buffer);
        assert!(undo3.is_some(), "Third undo should work correctly");
        assert_eq!(
            buffer.get_line(0).unwrap(),
            "Hello World",
            "Third undo should work correctly"
        );
    }
}

#[test]
fn test_insert_mode_group_add_char() {
    let mut group = rustvim::history::InsertModeGroup::new(rustvim::buffer::Position::new(0, 0));

    group.add_char('h');
    group.add_char('e');
    group.add_char('l');
    group.add_char('l');
    group.add_char('o');

    assert_eq!(group.inserted_text, "hello");
    assert!(group.has_changes());
}

#[test]
fn test_insert_mode_group_add_newline() {
    let mut group = rustvim::history::InsertModeGroup::new(rustvim::buffer::Position::new(0, 0));

    group.add_char('h');
    group.add_char('i');
    group.add_newline();

    assert_eq!(group.inserted_text, "hi\n");
}

#[test]
fn test_insert_mode_group_add_deleted_char() {
    let mut group = rustvim::history::InsertModeGroup::new(rustvim::buffer::Position::new(0, 0));

    group.add_deleted_char('x', rustvim::buffer::Position::new(0, 2));
    group.add_deleted_char('y', rustvim::buffer::Position::new(0, 3));

    assert_eq!(group.deleted_text, "xy");
    assert_eq!(
        group.deletion_start_pos,
        Some(rustvim::buffer::Position::new(0, 2))
    );
}
