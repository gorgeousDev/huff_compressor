

/// Unit tests for get_freqs function
///
///
#[cfg(test)]
mod get_freqs_tests {
    use crate::logic::encoding_binary_tree::get_freqs;

    #[test]
    fn test_empty_string() {
        let result = get_freqs("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_character() {
        let result = get_freqs("a");
        assert_eq!(result.get(&'a'), Some(&1));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_multiple_same_characters() {
        let result = get_freqs("aaa");
        assert_eq!(result.get(&'a'), Some(&3));
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_multiple_different_characters() {
        let result = get_freqs("aba");
        assert_eq!(result.get(&'a'), Some(&2));
        assert_eq!(result.get(&'b'), Some(&1));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_case_sensitivity() {
        let result = get_freqs("aAa");
        assert_eq!(result.get(&'a'), Some(&2));
        assert_eq!(result.get(&'A'), Some(&1));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_with_spaces() {
        let result = get_freqs("a b a");
        assert_eq!(result.get(&'a'), Some(&2));
        assert_eq!(result.get(&' '), Some(&2));
        assert_eq!(result.get(&'b'), Some(&1));
    }

    #[test]
    fn test_special_characters() {
        let result = get_freqs("!@#!@");
        assert_eq!(result.get(&'!'), Some(&2));
        assert_eq!(result.get(&'@'), Some(&2));
        assert_eq!(result.get(&'#'), Some(&1));
    }

    #[test]
    fn test_unicode_characters() {
        let result = get_freqs("héllo");
        assert_eq!(result.get(&'h'), Some(&1));
        assert_eq!(result.get(&'é'), Some(&1));
        assert_eq!(result.get(&'l'), Some(&2));
        assert_eq!(result.get(&'o'), Some(&1));
    }

    #[test]
    fn test_numbers_as_chars() {
        let result = get_freqs("123321");
        assert_eq!(result.get(&'1'), Some(&2));
        assert_eq!(result.get(&'2'), Some(&2));
        assert_eq!(result.get(&'3'), Some(&2));
    }

    #[test]
    fn test_newline_character() {
        let result = get_freqs("a\nb\na");
        assert_eq!(result.get(&'a'), Some(&2));
        assert_eq!(result.get(&'b'), Some(&1));
        assert_eq!(result.get(&'\n'), Some(&2));
    }

    #[test]
    fn test_long_string() {
        let text = "abcdefghijklmnopqrstuvwxyz".repeat(100);
        let result = get_freqs(&text);
        for c in 'a'..='z' {
            assert_eq!(result.get(&c), Some(&100));
        }
    }

    #[test]
    fn test_all_same_characters_long() {
        let text = "a".repeat(10000);
        let result = get_freqs(&text);
        assert_eq!(result.get(&'a'), Some(&10000));
        assert_eq!(result.len(), 1);
    }
}

/// Unit tests for print_help function
#[cfg(test)]
mod print_help_tests {
    use crate::logic::print_help;

    #[test]
    fn test_print_help_does_not_panic() {
        // print_help prints to stdout, so we just verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            print_help();
        });
        assert!(result.is_ok(), "print_help should not panic");
    }
}

