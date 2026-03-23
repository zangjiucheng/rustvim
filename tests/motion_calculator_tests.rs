#[cfg(test)]
mod motion_calculator_tests {
    use rustvim::commands::MotionCalculator;
    use rustvim::editor::Editor;

    #[test]
    fn test_word_forward_single_line() {
        let mut editor = Editor::new();
        editor
            .buffer_mut()
            .insert_line(0, "hello world test".to_string());

        let mut row = 0;
        let mut col = 0;
        MotionCalculator::word_forward(&editor, &mut row, &mut col);

        assert_eq!(row, 0);
        assert!(col > 0);
    }

    #[test]
    #[should_panic]
    fn test_word_forward_empty_line() {
        let mut editor = Editor::new();
        editor.buffer_mut().insert_line(0, "".to_string());

        let mut row = 0;
        let mut col = 0;
        MotionCalculator::word_forward(&editor, &mut row, &mut col);

        assert_eq!(row, 0);
    }

    #[test]
    fn test_word_backward_single_line() {
        let mut editor = Editor::new();
        editor
            .buffer_mut()
            .insert_line(0, "hello world test".to_string());

        let mut row = 0;
        let mut col = 10;
        MotionCalculator::word_backward(&editor, &mut row, &mut col);

        assert_eq!(row, 0);
        assert!(col < 10);
    }

    #[test]
    fn test_word_backward_empty_line() {
        let mut editor = Editor::new();
        editor.buffer_mut().insert_line(0, "".to_string());

        let mut row = 0;
        let mut col = 0;
        MotionCalculator::word_backward(&editor, &mut row, &mut col);

        assert_eq!(row, 0);
    }

    #[test]
    fn test_word_end_single_line() {
        let mut editor = Editor::new();
        editor
            .buffer_mut()
            .insert_line(0, "hello world".to_string());

        let mut row = 0;
        let mut col = 0;
        MotionCalculator::word_end(&editor, &mut row, &mut col);

        assert_eq!(row, 0);
    }

    #[test]
    #[should_panic]
    fn test_word_end_empty_line() {
        let mut editor = Editor::new();
        editor.buffer_mut().insert_line(0, "".to_string());

        let mut row = 0;
        let mut col = 0;
        MotionCalculator::word_end(&editor, &mut row, &mut col);

        assert_eq!(row, 0);
    }

    #[test]
    fn test_word_forward_at_end_of_line() {
        let mut editor = Editor::new();
        editor.buffer_mut().insert_line(0, "word1".to_string());
        editor.buffer_mut().insert_line(1, "word2".to_string());

        let mut row = 0;
        let mut col = 4;
        MotionCalculator::word_forward(&editor, &mut row, &mut col);

        assert_eq!(row, 1);
    }
}
