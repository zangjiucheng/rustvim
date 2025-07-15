use rustvim::editor::{Editor, Mode, BufferInfo, Cursor};
use rustvim::buffer::Buffer;
use rustvim::history::History;

#[test]
fn test_visual_mode_delete_redo_multiple_buffers() {
    // Create editor with multiple buffers
    let mut editor = Editor::new();
    
    // Buffer 1: Initial content
    let buffer1 = Buffer::from_file("line 1\nline 2\nline 3");
    editor.buffers[0].buffer = buffer1;
    editor.buffers[0].filename = Some("file1.txt".to_string());
    editor.buffers[0].modified = false;
    
    // Buffer 2: Different content
    let buffer_info2 = BufferInfo {
        buffer: Buffer::from_file("hello world\nfoo bar\nbaz qux"),
        filename: Some("file2.txt".to_string()),
        modified: false,
        cursor: Cursor::new(),
        scroll_offset: 0,
        history: History::new(),
    };
    editor.add_buffer(buffer_info2);
    
    // Buffer 3: More content
    let buffer_info3 = BufferInfo {
        buffer: Buffer::from_file("first\nsecond\nthird\nfourth"),
        filename: Some("file3.txt".to_string()),
        modified: false,
        cursor: Cursor::new(),
        scroll_offset: 0,
        history: History::new(),
    };
    editor.add_buffer(buffer_info3);
    
    // Now we have 3 buffers, currently on buffer 3 (index 2)
    assert_eq!(editor.buffers.len(), 3);
    assert_eq!(editor.current_buffer, 2);
    
    // === Test Buffer 3 (current): Visual line delete and redo ===
    
    // Move to line 1 (row 1 = "second")
    editor.move_cursor(1, 0);
    assert_eq!(editor.cursor().row, 1);
    
    // Enter visual line mode and select 2 lines ("second" and "third")
    editor.enter_visual_line_mode();
    assert_eq!(editor.mode, Mode::Visual);
    assert!(editor.visual_line_mode);
    
    // Extend selection down to include "third"
    editor.cursor_mut().row += 1;
    assert_eq!(editor.cursor().row, 2);
    
    // Delete the visual selection
    editor.delete_visual_selection().unwrap();
    
    // Verify deletion in buffer 3
    assert_eq!(editor.mode, Mode::Normal);
    assert!(!editor.visual_line_mode);
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "first");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "fourth");
    assert_eq!(editor.cursor().row, 1); // Cursor should be on "fourth"
    assert!(editor.is_modified());
    
    // Test undo in buffer 3
    editor.undo();
    assert_eq!(editor.buffer().line_count(), 4);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "first");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "second");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "third");
    assert_eq!(editor.buffer().get_line(3).unwrap(), "fourth");
    
    // Test redo in buffer 3
    editor.redo();
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "first");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "fourth");
    
    // === Switch to Buffer 2 and test visual character delete ===
    
    editor.switch_to_buffer(1);
    assert_eq!(editor.current_buffer, 1);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello world");
    
    // Position cursor at 'w' in "world"
    editor.move_cursor(0, 6);
    
    // Enter visual mode and select "wor"
    editor.enter_visual_mode();
    assert_eq!(editor.mode, Mode::Visual);
    assert!(!editor.visual_line_mode);
    
    // Extend selection to include "wor"
    editor.cursor_mut().col += 2; // now on 'r'
    
    // Delete the visual selection
    editor.delete_visual_selection().unwrap();
    
    // Verify deletion in buffer 2
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello ld");
    assert_eq!(editor.cursor().col, 6); // Cursor at deletion start
    assert!(editor.is_modified());
    
    // Test undo in buffer 2
    editor.undo();
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello world");
    
    // Test redo in buffer 2
    editor.redo();
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello ld");
    
    // === Switch to Buffer 1 and test visual line delete ===
    
    editor.switch_to_buffer(0);
    assert_eq!(editor.current_buffer, 0);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    
    // Move to middle line (row 1 = "line 2")
    editor.move_cursor(1, 0);
    
    // Enter visual line mode and delete single line
    editor.enter_visual_line_mode();
    editor.delete_visual_selection().unwrap();
    
    // Verify deletion in buffer 1
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 3");
    assert!(editor.is_modified());
    
    // Test undo in buffer 1
    editor.undo();
    assert_eq!(editor.buffer().line_count(), 3);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 2");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "line 3");
    
    // Test redo in buffer 1
    editor.redo();
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 3");
    
    // === Verify buffer independence ===
    
    // Switch back to buffer 3 and verify its state is preserved
    editor.switch_to_buffer(2);
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "first");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "fourth");
    
    // Test that buffer 3's undo history is still intact
    editor.undo();
    assert_eq!(editor.buffer().line_count(), 4);
    assert_eq!(editor.buffer().get_line(1).unwrap(), "second");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "third");
    
    // Switch to buffer 2 and verify its state
    editor.switch_to_buffer(1);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello ld");
    
    // Test that buffer 2's undo history is preserved
    editor.undo();
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello world");
    
    // Switch to buffer 1 and verify its state
    editor.switch_to_buffer(0);
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 3");
    
    // Test that buffer 1's undo history works
    editor.undo();
    assert_eq!(editor.buffer().line_count(), 3);
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 2");
}

#[test]
fn test_visual_mode_cross_buffer_undo_redo_independence() {
    // Create editor with 2 buffers
    let mut editor = Editor::new();
    
    // Buffer 1
    let buffer1 = Buffer::from_file("original text");
    editor.buffers[0].buffer = buffer1;
    editor.buffers[0].filename = Some("buffer1.txt".to_string());
    
    // Buffer 2
    let buffer_info2 = BufferInfo {
        buffer: Buffer::from_file("different content"),
        filename: Some("buffer2.txt".to_string()),
        modified: false,
        cursor: Cursor::new(),
        scroll_offset: 0,
        history: History::new(),
    };
    editor.add_buffer(buffer_info2);
    
    // === Perform operations in buffer 2 (current) ===
    
    editor.move_cursor(0, 0);
    editor.enter_visual_mode();
    editor.cursor_mut().col += 9; // select "different " (including space)
    editor.delete_visual_selection().unwrap();
    
    assert_eq!(editor.buffer().get_line(0).unwrap(), "content");
    assert!(editor.is_modified());
    
    // === Switch to buffer 1 and perform different operations ===
    
    editor.switch_to_buffer(0);
    editor.move_cursor(0, 0);
    editor.enter_visual_mode();
    editor.cursor_mut().col += 8; // select "original " (8 chars + 1 space)
    editor.delete_visual_selection().unwrap();
    
    assert_eq!(editor.buffer().get_line(0).unwrap(), "text");
    assert!(editor.is_modified());
    
    // === Test independent undo/redo ===
    
    // Undo in buffer 1
    editor.undo();
    assert_eq!(editor.buffer().get_line(0).unwrap(), "original text");
    
    // Switch to buffer 2 - its state should be unchanged
    editor.switch_to_buffer(1);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "content"); // Still deleted
    
    // Undo in buffer 2
    editor.undo();
    assert_eq!(editor.buffer().get_line(0).unwrap(), "different content");
    
    // Switch back to buffer 1 - its undo state should be preserved
    editor.switch_to_buffer(0);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "original text"); // Still undone
    
    // Redo in buffer 1
    editor.redo();
    assert_eq!(editor.buffer().get_line(0).unwrap(), "text");
    
    // Switch to buffer 2 and redo
    editor.switch_to_buffer(1);
    editor.redo();
    assert_eq!(editor.buffer().get_line(0).unwrap(), "content");
    
    // === Verify final independent states ===
    
    // Buffer 1 should show redone state
    editor.switch_to_buffer(0);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "text");
    
    // Buffer 2 should show redone state
    editor.switch_to_buffer(1);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "content");
}

#[test] 
fn test_visual_mode_delete_redo_with_buffer_switching_stress() {
    // Test rapid buffer switching with visual operations and undo/redo
    let mut editor = Editor::new();
    
    // Create 3 buffers with different content
    for i in 0..3 {
        let content = format!("buffer {} line 1\nbuffer {} line 2\nbuffer {} line 3", i, i, i);
        
        if i == 0 {
            let buffer = Buffer::from_file(&content);
            editor.buffers[0].buffer = buffer;
            editor.buffers[0].filename = Some(format!("buffer{}.txt", i));
        } else {
            let buffer_info = BufferInfo {
                buffer: Buffer::from_file(&content),
                filename: Some(format!("buffer{}.txt", i)),
                modified: false,
                cursor: Cursor::new(),
                scroll_offset: 0,
                history: History::new(),
            };
            editor.add_buffer(buffer_info);
        }
    }
    
    // Perform visual operations in each buffer
    for buffer_idx in 0..3 {
        editor.switch_to_buffer(buffer_idx);
        
        // Move to line 1 and perform visual line delete
        editor.move_cursor(1, 0);
        editor.enter_visual_line_mode();
        editor.delete_visual_selection().unwrap();
        
        // Verify the deletion
        assert_eq!(editor.buffer().line_count(), 2);
        assert_eq!(editor.buffer().get_line(0).unwrap(), &format!("buffer {} line 1", buffer_idx));
        assert_eq!(editor.buffer().get_line(1).unwrap(), &format!("buffer {} line 3", buffer_idx));
    }
    
    // Now test undo in each buffer
    for buffer_idx in 0..3 {
        editor.switch_to_buffer(buffer_idx);
        
        // Should be able to undo
        assert!(editor.history().can_undo());
        editor.undo();
        
        // Verify restoration
        assert_eq!(editor.buffer().line_count(), 3);
        assert_eq!(editor.buffer().get_line(1).unwrap(), &format!("buffer {} line 2", buffer_idx));
    }
    
    // Test redo in reverse order
    for buffer_idx in (0..3).rev() {
        editor.switch_to_buffer(buffer_idx);
        
        // Should be able to redo
        assert!(editor.history().can_redo());
        editor.redo();
        
        // Verify deletion is back
        assert_eq!(editor.buffer().line_count(), 2);
        assert_eq!(editor.buffer().get_line(0).unwrap(), &format!("buffer {} line 1", buffer_idx));
        assert_eq!(editor.buffer().get_line(1).unwrap(), &format!("buffer {} line 3", buffer_idx));
    }
    
    // Final verification: each buffer should maintain independent history
    for buffer_idx in 0..3 {
        editor.switch_to_buffer(buffer_idx);
        
        // Each buffer should have exactly 1 undo available (the redo we just did)
        assert!(editor.history().can_undo());
        assert_eq!(editor.history().undo_count(), 1);
        
        // No more redos available
        assert!(!editor.history().can_redo());
        assert_eq!(editor.history().redo_count(), 0);
    }
}
