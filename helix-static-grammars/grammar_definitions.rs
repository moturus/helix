macro_rules! grammar_definitions {
    ($callback:ident) => {
        $callback! {
            ("rust", tree_sitter_rust, "77a3747266f4d621d0757825e6b11edcbf991ca5", "rust", "", C),
            ("toml", tree_sitter_toml, "7cff70bbcbbc62001b465603ca1ea88edd668704", "toml", "", C),
            ("markdown", tree_sitter_markdown, "62516e8c78380e3b51d5b55727995d2c511436d8", "markdown", "tree-sitter-markdown", C),
            ("markdown_inline", tree_sitter_markdown_inline, "62516e8c78380e3b51d5b55727995d2c511436d8", "markdown", "tree-sitter-markdown-inline", C),
            ("html", tree_sitter_html, "cbb91a0ff3621245e890d1c50cc811bffb77a26b", "html", "", C),
            ("c", tree_sitter_c, "7175a6dd5fc1cee660dce6fe23f6043d75af424a", "c", "", None),
            ("cpp", tree_sitter_cpp, "56455f4245baf4ea4e0881c5169de69d7edd5ae7", "cpp", "", C),
            ("json", tree_sitter_json, "73076754005a460947cafe8e03a8cf5fa4fa2938", "json", "", None),
            ("yaml", tree_sitter_yaml, "0e36bed171768908f331ff7dff9d956bae016efb", "yaml", "", Cpp),
            ("bash", tree_sitter_bash, "487734f87fd87118028a65a4599352fa99c9cde8", "bash", "", C),
            ("lua", tree_sitter_lua, "88e446476a1e97a8724dff7a23e2d709855077f2", "lua", "", C),
        }
    };
}
