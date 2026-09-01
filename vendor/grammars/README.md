# Vendored tree-sitter grammars

This directory contains only the generated sources, required headers, and
licenses needed to build Helix's curated grammar set. The files were copied
from the exact upstream revisions below; upstream Git metadata and unrelated
bindings, tests, documentation, and build artifacts are intentionally absent.

| Directory | Upstream | Revision |
| --- | --- | --- |
| `rust` | `tree-sitter/tree-sitter-rust` | `77a3747266f4d621d0757825e6b11edcbf991ca5` |
| `toml` | `ikatyang/tree-sitter-toml` | `7cff70bbcbbc62001b465603ca1ea88edd668704` |
| `markdown` | `tree-sitter-grammars/tree-sitter-markdown` | `62516e8c78380e3b51d5b55727995d2c511436d8` |
| `c` | `tree-sitter/tree-sitter-c` | `7175a6dd5fc1cee660dce6fe23f6043d75af424a` |
| `cpp` | `tree-sitter/tree-sitter-cpp` | `56455f4245baf4ea4e0881c5169de69d7edd5ae7` |
| `json` | `tree-sitter/tree-sitter-json` | `73076754005a460947cafe8e03a8cf5fa4fa2938` |
| `yaml` | `ikatyang/tree-sitter-yaml` | `0e36bed171768908f331ff7dff9d956bae016efb` |
| `bash` | `tree-sitter/tree-sitter-bash` | `487734f87fd87118028a65a4599352fa99c9cde8` |
| `lua` | `tree-sitter-grammars/tree-sitter-lua` | `88e446476a1e97a8724dff7a23e2d709855077f2` |

`markdown` contains both the block and inline grammar subdirectories. Each
`REVISION` file is checked by the host grammar builder and the static grammar
crate before compiling any vendored source.
