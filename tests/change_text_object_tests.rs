#[cfg(test)]
mod find_char_motion_tests {
    use rustvim::commands::{Motion, MovementCommand, MovementExecutor};
    use rustvim::editor::Editor;

    #[test]
    fn test_find_char_forward_basic() {
        let mut editor = Editor::new();
        editor
            .buffer_mut()
            .insert_line(0, "Hello World".to_string());
        editor.cursor_mut().col = 0;

        MovementExecutor::execute_movement(&mut editor, MovementCommand::FindCharForward('l'), 1);

        assert_eq!(editor.cursor().col, 2);
    }

    #[test]
    fn test_find_char_forward_not_found() {
        let mut editor = Editor::new();
        editor
            .buffer_mut()
            .insert_line(0, "Hello World".to_string());
        editor.cursor_mut().col = 0;

        MovementExecutor::execute_movement(&mut editor, MovementCommand::FindCharForward('z'), 1);

        assert_eq!(editor.cursor().col, 0);
    }

    #[test]
    fn test_find_char_backward_basic() {
        let mut editor = Editor::new();
        editor
            .buffer_mut()
            .insert_line(0, "Hello World".to_string());
        editor.cursor_mut().col = 9;

        MovementExecutor::execute_movement(&mut editor, MovementCommand::FindCharBackward('l'), 1);

        assert!(editor.cursor().col <= 9);
    }

    #[test]
    fn test_till_char_forward() {
        let mut editor = Editor::new();
        editor
            .buffer_mut()
            .insert_line(0, "Hello World".to_string());
        editor.cursor_mut().col = 0;

        MovementExecutor::execute_movement(&mut editor, MovementCommand::TillCharForward('l'), 1);

        assert!(editor.cursor().col < 2);
    }

    #[test]
    fn test_find_char_movement_is_not_line_motion() {
        let motion = MovementCommand::FindCharForward('a');
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_till_char_movement_is_not_line_motion() {
        let motion = MovementCommand::TillCharForward('a');
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_repeat_find_forward_movement_exists() {
        let motion = MovementCommand::RepeatFindForward;
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_repeat_find_backward_movement_exists() {
        let motion = MovementCommand::RepeatFindBackward;
        assert!(!motion.is_line_motion());
    }
}

#[cfg(test)]
mod text_object_motion_tests {
    use rustvim::commands::{Motion, MovementCommand, TextObject};
    use rustvim::editor::Editor;

    #[test]
    fn test_inner_word_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::InnerWord);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_around_word_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::AroundWord);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_inner_quote_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::InnerQuote);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_around_quote_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::AroundQuote);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_inner_paren_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::InnerParen);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_around_paren_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::AroundParen);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_inner_bracket_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::InnerBracket);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_inner_brace_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::InnerBrace);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_inner_angle_motion_exists() {
        let motion = MovementCommand::TextObject(TextObject::InnerAngle);
        assert!(!motion.is_line_motion());
    }

    #[test]
    fn test_inner_word_motion_calculates_position() {
        let editor = Editor::new();
        let motion = MovementCommand::TextObject(TextObject::InnerWord);
        let result = motion.calculate_end_position(&editor, (0, 0), 1);
        assert_eq!(result.0, 0);
    }

    #[test]
    fn test_inner_quote_motion_calculates_position() {
        let editor = Editor::new();
        let motion = MovementCommand::TextObject(TextObject::InnerQuote);
        let result = motion.calculate_end_position(&editor, (0, 0), 1);
        assert_eq!(result.0, 0);
    }

    #[test]
    fn test_inner_paren_motion_calculates_position() {
        let editor = Editor::new();
        let motion = MovementCommand::TextObject(TextObject::InnerParen);
        let result = motion.calculate_end_position(&editor, (0, 0), 1);
        assert_eq!(result.0, 0);
    }
}

#[cfg(test)]
mod pending_action_tests {
    use rustvim::keymap::PendingAction;

    #[test]
    fn test_delete_pending_action_exists() {
        let action = PendingAction::Delete;
        assert!(matches!(action, PendingAction::Delete));
    }

    #[test]
    fn test_yank_pending_action_exists() {
        let action = PendingAction::Yank;
        assert!(matches!(action, PendingAction::Yank));
    }

    #[test]
    fn test_change_pending_action_exists() {
        let action = PendingAction::Change;
        assert!(matches!(action, PendingAction::Change));
    }

    #[test]
    fn test_delete_inner_object_pending_exists() {
        let action = PendingAction::DeleteInnerObject;
        assert!(matches!(action, PendingAction::DeleteInnerObject));
    }

    #[test]
    fn test_delete_around_object_pending_exists() {
        let action = PendingAction::DeleteAroundObject;
        assert!(matches!(action, PendingAction::DeleteAroundObject));
    }

    #[test]
    fn test_yank_inner_object_pending_exists() {
        let action = PendingAction::YankInnerObject;
        assert!(matches!(action, PendingAction::YankInnerObject));
    }

    #[test]
    fn test_yank_around_object_pending_exists() {
        let action = PendingAction::YankAroundObject;
        assert!(matches!(action, PendingAction::YankAroundObject));
    }

    #[test]
    fn test_change_inner_object_pending_exists() {
        let action = PendingAction::ChangeInnerObject;
        assert!(matches!(action, PendingAction::ChangeInnerObject));
    }

    #[test]
    fn test_change_around_object_pending_exists() {
        let action = PendingAction::ChangeAroundObject;
        assert!(matches!(action, PendingAction::ChangeAroundObject));
    }

    #[test]
    fn test_find_char_forward_pending_exists() {
        let action = PendingAction::FindCharForward;
        assert!(matches!(action, PendingAction::FindCharForward));
    }

    #[test]
    fn test_find_char_backward_pending_exists() {
        let action = PendingAction::FindCharBackward;
        assert!(matches!(action, PendingAction::FindCharBackward));
    }

    #[test]
    fn test_till_char_forward_pending_exists() {
        let action = PendingAction::TillCharForward;
        assert!(matches!(action, PendingAction::TillCharForward));
    }

    #[test]
    fn test_till_char_backward_pending_exists() {
        let action = PendingAction::TillCharBackward;
        assert!(matches!(action, PendingAction::TillCharBackward));
    }
}
