use std::env;
use std::fs;
use std::path::{Path, PathBuf};

include!("grammar_definitions.rs");

#[derive(Clone, Copy)]
enum Scanner {
    None,
    C,
    Cpp,
}

struct GrammarDefinition {
    name: &'static str,
    revision: &'static str,
    vendor: &'static str,
    subpath: &'static str,
    scanner: Scanner,
}

macro_rules! define_build_grammars {
    ($(($name:literal, $symbol:ident, $revision:literal, $vendor:literal, $subpath:literal, $scanner:ident),)*) => {
        const GRAMMARS: &[GrammarDefinition] = &[
            $(GrammarDefinition {
                name: $name,
                revision: $revision,
                vendor: $vendor,
                subpath: $subpath,
                scanner: Scanner::$scanner,
            },)*
        ];
    };
}

grammar_definitions!(define_build_grammars);

fn main() {
    println!("cargo:rerun-if-changed=grammar_definitions.rs");
    if env::var_os("CARGO_FEATURE_STATIC_GRAMMARS").is_none() {
        return;
    }

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("static grammar crate must be in the Helix workspace");
    for grammar in GRAMMARS {
        build_grammar(workspace, grammar);
    }
}

fn build_grammar(workspace: &Path, grammar: &GrammarDefinition) {
    let vendor_dir = workspace.join("vendor/grammars").join(grammar.vendor);
    let revision_path = vendor_dir.join("REVISION");
    println!("cargo:rerun-if-changed={}", revision_path.display());
    let observed = fs::read_to_string(&revision_path).unwrap_or_else(|error| {
        panic!(
            "grammar {}: cannot read revision marker {}: {error}",
            grammar.name,
            revision_path.display()
        )
    });
    assert_eq!(
        observed.trim(),
        grammar.revision,
        "grammar {}: expected revision {}, observed {} in {}",
        grammar.name,
        grammar.revision,
        observed.trim(),
        revision_path.display()
    );

    let source_dir = vendor_dir.join(grammar.subpath).join("src");
    let parser = required_source(grammar.name, &source_dir, "parser.c");
    compile_c(grammar.name, "parser", &source_dir, &parser);
    match grammar.scanner {
        Scanner::None => {}
        Scanner::C => {
            let scanner = required_source(grammar.name, &source_dir, "scanner.c");
            compile_c(grammar.name, "scanner", &source_dir, &scanner);
        }
        Scanner::Cpp => {
            let scanner = required_source(grammar.name, &source_dir, "scanner.cc");
            compile_cpp(grammar.name, &source_dir, &scanner);
        }
    }
}

fn required_source(grammar: &str, source_dir: &Path, filename: &str) -> PathBuf {
    let path = source_dir.join(filename);
    assert!(
        path.is_file(),
        "grammar {grammar}: required vendored source is missing: {}",
        path.display()
    );
    println!("cargo:rerun-if-changed={}", path.display());
    path
}

fn compile_c(grammar: &str, unit: &str, source_dir: &Path, source: &Path) {
    cc::Build::new()
        .warnings(false)
        .include(source_dir)
        .file(source)
        .compile(&format!("helix_grammar_{grammar}_{unit}"));
}

fn compile_cpp(grammar: &str, source_dir: &Path, source: &Path) {
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .include(source_dir)
        .file(source)
        .compile(&format!("helix_grammar_{grammar}_scanner"));
}
