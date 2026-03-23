#[cfg(test)]
mod key_enum_tests {
    use rustvim::input::Key;

    #[test]
    fn test_key_is_char() {
        assert!(Key::Char('a').is_char());
        assert!(Key::Char('z').is_char());
        assert!(Key::Char('0').is_char());
        assert!(Key::Char(' ').is_char());

        assert!(!Key::Ctrl('c').is_char());
        assert!(!Key::Esc.is_char());
        assert!(!Key::Up.is_char());
        assert!(!Key::Enter.is_char());
    }

    #[test]
    fn test_key_is_ctrl() {
        assert!(Key::Ctrl('c').is_ctrl());
        assert!(Key::Ctrl('a').is_ctrl());
        assert!(Key::Ctrl('z').is_ctrl());

        assert!(!Key::Char('c').is_ctrl());
        assert!(!Key::Esc.is_ctrl());
        assert!(!Key::Up.is_ctrl());
    }

    #[test]
    fn test_key_is_arrow() {
        assert!(Key::Up.is_arrow());
        assert!(Key::Down.is_arrow());
        assert!(Key::Left.is_arrow());
        assert!(Key::Right.is_arrow());

        assert!(!Key::Char('a').is_arrow());
        assert!(!Key::Esc.is_arrow());
        assert!(!Key::Home.is_arrow());
    }

    #[test]
    fn test_key_as_char() {
        assert_eq!(Key::Char('a').as_char(), Some('a'));
        assert_eq!(Key::Char('z').as_char(), Some('z'));
        assert_eq!(Key::Char('0').as_char(), Some('0'));
        assert_eq!(Key::Char(' ').as_char(), Some(' '));

        assert_eq!(Key::Ctrl('c').as_char(), None);
        assert_eq!(Key::Esc.as_char(), None);
        assert_eq!(Key::Up.as_char(), None);
        assert_eq!(Key::Enter.as_char(), None);
    }

    #[test]
    fn test_key_enum_variants() {
        let keys = vec![
            Key::Char('a'),
            Key::Ctrl('c'),
            Key::Esc,
            Key::Enter,
            Key::Backspace,
            Key::Delete,
            Key::Tab,
            Key::Up,
            Key::Down,
            Key::Left,
            Key::Right,
            Key::Home,
            Key::End,
            Key::PageUp,
            Key::PageDown,
            Key::Function(1),
            Key::Function(12),
            Key::Unknown,
            Key::Timeout,
        ];

        for key in keys {
            let _ = format!("{:?}", key);
        }
    }
}
