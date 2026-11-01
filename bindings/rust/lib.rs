//! This crate provides CSV, PSV, & TSV language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! "Name","Age","Salary"
//! "John Doe",30,120000
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! parser.set_language(&tree_sitter_csv::language_csv()).expect("Error loading CSV grammar");
//! let tree = parser.parse(code, None).unwrap();
//!
//! let code = r"
//! Name|Age|Salary
//! John Doe|30|120000
//! ";
//! let mut parser = tree_sitter::Parser::new();
//! parser.set_language(&tree_sitter_csv::language_tsv()).expect("Error loading PSV grammar");
//! let tree = parser.parse(code, None).unwrap();
//!
//! let code = r"
//! Name\tAge\tSalary
//! John Doe\t30\t120000
//! ";
//! let mut parser = tree_sitter::Parser::new();
//! parser.set_language(&tree_sitter_csv::language_tsv()).expect("Error loading TSV grammar");
//! let tree = parser.parse(code, None).unwrap();
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_csv() -> Language;
    fn tree_sitter_psv() -> Language;
    fn tree_sitter_tsv() -> Language;
}

/// Get the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_csv() -> Language {
    unsafe { tree_sitter_csv() }
}

pub fn language_psv() -> Language {
    unsafe { tree_sitter_psv() }
}

pub fn language_tsv() -> Language {
    unsafe { tree_sitter_tsv() }
}

/// The content of the [`grammar.json`][] file for CSV.
pub const GRAMMAR_JSON_CSV: &str = include_str!("../../csv/src/grammar.json");

/// The content of the [`grammar.json`][] file for PSV.
pub const GRAMMAR_JSON_PSV: &str = include_str!("../../psv/src/grammar.json");

/// The content of the [`grammar.json`][] file for TSV.
pub const GRAMMAR_JSON_TSV: &str = include_str!("../../tsv/src/grammar.json");

/// The content of the [`node-types.json`][] file for CSV.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_CSV: &str = include_str!("../../csv/src/node-types.json");

/// The content of the [`node-types.json`][] file for PSV.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_PSV: &str = include_str!("../../psv/src/node-types.json");

/// The content of the [`node-types.json`][] file for TSV.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_TSV: &str = include_str!("../../tsv/src/node-types.json");

/// The syntax highlighting query for CSV.
pub const HIGHLIGHT_QUERY_CSV: &str = include_str!("../../csv/queries/highlights.scm");

/// The syntax highlighting query for PSV.
pub const HIGHLIGHT_QUERY_PSV: &str = include_str!("../../psv/queries/highlights.scm");

/// The syntax highlighting query for TSV.
pub const HIGHLIGHT_QUERY_TSV: &str = include_str!("../../tsv/queries/highlights.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        for (language, highlights) in [
            (super::language_csv(), super::HIGHLIGHT_QUERY_CSV),
            (super::language_psv(), super::HIGHLIGHT_QUERY_PSV),
            (super::language_tsv(), super::HIGHLIGHT_QUERY_TSV),
        ] {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&language)
                .expect("Error loading grammar");
            tree_sitter::Query::new(&language, highlights).expect("Invalid highlighting query");
        }
    }

    #[test]
    fn test_field_types_and_row_boundaries() {
        for (language, separator) in [
            (super::language_csv(), ","),
            (super::language_psv(), "|"),
            (super::language_tsv(), "\t"),
        ] {
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(&language).unwrap();

            for newline in ["\n", "\r\n", "\r"] {
                let first = [
                    "text",
                    "-12",
                    "+.5",
                    "1e3",
                    "true",
                    "2026-11-01T12:34:56Z",
                    "",
                ]
                .join(separator);
                let input = format!(
                    "{first}{newline}{}",
                    ["next", "", "123abc", ""].join(separator)
                );
                let mut tree = parser.parse(&input, None).unwrap();
                let root = tree.root_node();
                assert!(!root.has_error(), "{}", root.to_sexp());
                assert_eq!(root.named_child_count(), 2);

                let first_row = root.named_child(0).unwrap();
                let expected = [
                    "text", "number", "float", "float", "boolean", "date", "text",
                ];
                assert_eq!(first_row.named_child_count(), expected.len());
                for (index, kind) in expected.iter().enumerate() {
                    let field = first_row.named_child(index).unwrap();
                    assert_eq!(field.named_child(0).unwrap().kind(), *kind);
                }

                let second_row = root.named_child(1).unwrap();
                assert_eq!(second_row.named_child_count(), 4);
                let empty = second_row.named_child(1).unwrap();
                assert_eq!(empty.start_byte(), empty.end_byte());

                let end = root.end_position();
                for (suffix, kind) in [("false", "boolean"), ("", "text")] {
                    let updated = format!("{input}{suffix}");
                    tree.edit(&tree_sitter::InputEdit {
                        start_byte: input.len(),
                        old_end_byte: tree.root_node().end_byte(),
                        new_end_byte: updated.len(),
                        start_position: end,
                        old_end_position: tree.root_node().end_position(),
                        new_end_position: tree_sitter::Point {
                            row: end.row,
                            column: end.column + suffix.len(),
                        },
                    });
                    tree = parser.parse(updated, Some(&tree)).unwrap();
                    let root = tree.root_node();
                    assert!(!root.has_error(), "{}", root.to_sexp());
                    let row = root.named_child(1).unwrap();
                    assert_eq!(row.named_child_count(), 4);
                    let last = row.named_child(3).unwrap();
                    assert_eq!(last.named_child(0).unwrap().kind(), kind);
                }
            }
        }
    }
}
