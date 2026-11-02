use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Added,
    Removed,
    Unchanged,
}

#[derive(Debug, Clone)]
pub struct WordChange {
    pub text: String,
    pub change_type: ChangeType,
}

#[must_use]
pub fn word_diff(old_line: &str, new_line: &str) -> (Vec<WordChange>, Vec<WordChange>) {
    let old_words: Vec<&str> = old_line.split_word_bounds().collect();
    let new_words: Vec<&str> = new_line.split_word_bounds().collect();

    let diff = similar::capture_diff_slices(similar::Algorithm::Myers, &old_words, &new_words);

    let mut old_changes = Vec::new();
    let mut new_changes = Vec::new();

    for change in diff {
        match change {
            similar::DiffOp::Equal {
                old_index,
                new_index: _,
                len,
            } => {
                for i in 0..len {
                    let word = old_words[old_index + i].to_string();
                    old_changes.push(WordChange {
                        text: word.clone(),
                        change_type: ChangeType::Unchanged,
                    });
                    new_changes.push(WordChange {
                        text: word,
                        change_type: ChangeType::Unchanged,
                    });
                }
            }
            similar::DiffOp::Delete {
                old_index,
                old_len,
                new_index: _,
            } => {
                for i in 0..old_len {
                    let word = old_words[old_index + i].to_string();
                    old_changes.push(WordChange {
                        text: word,
                        change_type: ChangeType::Removed,
                    });
                }
            }
            similar::DiffOp::Insert {
                old_index: _,
                new_index,
                new_len,
            } => {
                for i in 0..new_len {
                    let word = new_words[new_index + i].to_string();
                    new_changes.push(WordChange {
                        text: word,
                        change_type: ChangeType::Added,
                    });
                }
            }
            similar::DiffOp::Replace {
                old_index,
                old_len,
                new_index,
                new_len,
            } => {
                for i in 0..old_len {
                    let word = old_words[old_index + i].to_string();
                    old_changes.push(WordChange {
                        text: word,
                        change_type: ChangeType::Removed,
                    });
                }
                for i in 0..new_len {
                    let word = new_words[new_index + i].to_string();
                    new_changes.push(WordChange {
                        text: word,
                        change_type: ChangeType::Added,
                    });
                }
            }
        }
    }
    (old_changes, new_changes)
}

#[must_use]
pub fn colorize_word_changes(changes: &[WordChange]) -> String {
    let mut output = String::new();
    for wc in changes {
        match wc.change_type {
            ChangeType::Added => {
                output.push_str("\x1b[42m");
                output.push_str(&wc.text);
                output.push_str("\x1b[0m");
            }
            ChangeType::Removed => {
                output.push_str("\x1b[41m");
                output.push_str(&wc.text);
                output.push_str("\x1b[0m");
            }
            ChangeType::Unchanged => {
                output.push_str(&wc.text);
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_diff_identical_lines_are_unchanged() {
        let (old, new) = word_diff("hello world", "hello world");
        assert!(
            old.iter()
                .all(|c| matches!(c.change_type, ChangeType::Unchanged))
        );
        assert!(
            new.iter()
                .all(|c| matches!(c.change_type, ChangeType::Unchanged))
        );
        let joined: String = old.iter().map(|c| c.text.as_str()).collect();
        assert_eq!(joined, "hello world");
    }

    #[test]
    fn word_diff_marks_added_and_removed_words() {
        let (old, new) = word_diff("hello world", "goodbye world");
        assert!(
            old.iter()
                .any(|c| matches!(c.change_type, ChangeType::Removed)),
            "old side should contain removals"
        );
        assert!(
            new.iter()
                .any(|c| matches!(c.change_type, ChangeType::Added)),
            "new side should contain additions"
        );
    }

    #[test]
    fn colorize_word_changes_emits_ansi() {
        let (_, new) = word_diff("foo bar", "foo baz");
        let colored = colorize_word_changes(&new);
        assert_ne!(colored.len(), 0);
        assert!(colored.contains('\u{1b}'), "expected ANSI escape codes");
        assert!(colored.contains("foo"));
    }
}
