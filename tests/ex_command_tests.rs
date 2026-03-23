#[cfg(test)]
mod ex_command_tests {
    use rustvim::commands::{ExCommand, ExCommandParser};

    #[test]
    fn test_parse_write() {
        let cmd = ExCommandParser::parse("w");
        assert!(matches!(cmd, ExCommand::Write { filename: None }));

        let cmd = ExCommandParser::parse("w test.txt");
        assert!(matches!(cmd, ExCommand::Write { filename: Some(f) } if f == "test.txt"));
    }

    #[test]
    fn test_parse_quit() {
        let cmd = ExCommandParser::parse("q");
        assert!(matches!(cmd, ExCommand::Quit { force: false }));

        let cmd = ExCommandParser::parse("q!");
        assert!(matches!(cmd, ExCommand::Quit { force: true }));
    }

    #[test]
    fn test_parse_edit() {
        let cmd = ExCommandParser::parse("e");
        assert!(matches!(cmd, ExCommand::Unknown { .. }));

        let cmd = ExCommandParser::parse("e test.txt");
        assert!(matches!(cmd, ExCommand::Edit { filename } if filename == "test.txt"));
    }

    #[test]
    fn test_parse_empty() {
        let cmd = ExCommandParser::parse("");
        assert!(matches!(cmd, ExCommand::Unknown { .. }));
    }

    #[test]
    fn test_parse_buffer_commands() {
        let cmd = ExCommandParser::parse("bn");
        assert!(matches!(cmd, ExCommand::BufferNext));

        let cmd = ExCommandParser::parse("bp");
        assert!(matches!(cmd, ExCommand::BufferPrev));

        let cmd = ExCommandParser::parse("ls");
        assert!(matches!(cmd, ExCommand::BufferList));

        let cmd = ExCommandParser::parse("2");
        assert!(matches!(cmd, ExCommand::BufferNumber { number: 2 }));
    }

    #[test]
    fn test_parse_quit_all() {
        let cmd = ExCommandParser::parse("qa");
        assert!(matches!(cmd, ExCommand::QuitAll { force: false }));

        let cmd = ExCommandParser::parse("qa!");
        assert!(matches!(cmd, ExCommand::QuitAll { force: true }));
    }

    #[test]
    fn test_parse_write_quit() {
        let cmd = ExCommandParser::parse("wq");
        assert!(matches!(cmd, ExCommand::WriteQuit { force: false }));
    }

    #[test]
    fn test_parse_unknown() {
        let cmd = ExCommandParser::parse("unknowncmd");
        assert!(matches!(cmd, ExCommand::Unknown { .. }));
    }
}
