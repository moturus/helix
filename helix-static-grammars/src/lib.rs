use tree_sitter_language::LanguageFn;

include!("../grammar_definitions.rs");

macro_rules! define_names {
    ($(($name:literal, $symbol:ident, $revision:literal, $vendor:literal, $subpath:literal, $scanner:ident),)*) => {
        pub const NAMES: &[&str] = &[$($name,)*];
    };
}

grammar_definitions!(define_names);

#[cfg(feature = "static-grammars")]
macro_rules! define_static_grammars {
    ($(($name:literal, $symbol:ident, $revision:literal, $vendor:literal, $subpath:literal, $scanner:ident),)*) => {
        unsafe extern "C" {
            $(fn $symbol() -> *const ();)*
        }

        fn get_static(name: &str) -> Option<LanguageFn> {
            match name {
                $($name => {
                    // The symbol is emitted by the revision-checked tree-sitter parser above.
                    Some(unsafe { LanguageFn::from_raw($symbol) })
                },)*
                _ => None,
            }
        }
    };
}

#[cfg(feature = "static-grammars")]
grammar_definitions!(define_static_grammars);

pub fn get(name: &str) -> Option<LanguageFn> {
    #[cfg(feature = "static-grammars")]
    return get_static(name);
    #[cfg(not(feature = "static-grammars"))]
    {
        let _ = name;
        None
    }
}

#[cfg(all(test, not(feature = "static-grammars")))]
mod feature_free_tests {
    use super::{get, NAMES};

    #[test]
    fn registry_is_inert_without_the_feature() {
        assert_eq!(NAMES.len(), 11);
        assert!(get("rust").is_none());
    }
}

#[cfg(all(test, feature = "static-grammars"))]
mod tests {
    use super::{get, NAMES};
    use ropey::Rope;
    use tree_house::tree_sitter::{Grammar, Node, Parser};

    const FIXTURES: &[(&str, &str)] = &[
        ("rust", "fn main() { let answer = 42; }\n"),
        ("toml", "name = \"motor\"\n"),
        ("markdown", "# Heading\n\nText.\n"),
        ("markdown_inline", "**bold** text\n"),
        ("html", "<!doctype html><strong>Motor</strong>\n"),
        ("c", "int main(void) { return 0; }\n"),
        ("cpp", "int main() { return 0; }\n"),
        ("json", "{\"ready\": true}\n"),
        ("yaml", "ready: true\n"),
        ("bash", "echo hello\n"),
        ("lua", "local answer = 42\n"),
    ];

    fn assert_valid_tree(node: Node<'_>) {
        assert_ne!(node.kind(), "ERROR");
        assert!(!node.is_missing(), "missing {} node", node.kind());
        for child in node.children() {
            assert_valid_tree(child);
        }
    }

    #[test]
    fn curated_names_resolve_and_parse() {
        assert_eq!(
            NAMES,
            FIXTURES.iter().map(|(name, _)| *name).collect::<Vec<_>>()
        );
        for &(name, source) in FIXTURES {
            let grammar = Grammar::try_from(get(name).expect("curated grammar must resolve"))
                .expect("grammar ABI must be compatible");
            let mut parser = Parser::new();
            parser.set_grammar(grammar).unwrap();
            let source = Rope::from_str(source);
            let tree = parser
                .parse(source.slice(..), None)
                .expect("parse must finish");
            assert_valid_tree(tree.root_node());
        }
    }

    #[test]
    fn unknown_name_is_rejected() {
        assert!(get("not-curated").is_none());
    }
}
