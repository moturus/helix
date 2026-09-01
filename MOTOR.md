# Motor OS port

This branch ports Helix 25.07.1 to Motor OS. It is based directly on upstream
commit `a05c151bb6e8e9c65ec390b0ae2afe7a5efd619b`. The base commit identifies the
upstream release; the final commit is the full commit hash of this Motor fork
after all porting changes and exact dependency pins have landed.

Motor artifacts are cross-compiled on a Linux host with the Motor OS
repository-selected toolchain. Building Helix natively on Motor OS is not
supported or tested.

## Dependency acquisition

Cargo dependency acquisition is an explicit online setup action:

```sh
cargo +1.90.0 fetch --locked
```

The ten curated grammars are revision-checked sources under
`vendor/grammars`; grammar acquisition is not part of a build. The first host
build compiles shared grammar libraries from those sources. Later builds reuse
them until their inputs change. All Cargo build, check, and test commands after
dependency acquisition use both `--locked` and `--offline`.

## Host checks

Run host commands from the root of this checkout so Cargo reads
`.cargo/config.toml`:

```sh
cargo +1.90.0 test --workspace --locked --offline

cargo +1.90.0 clippy --workspace --all-targets --locked --offline

cargo +1.90.0 fmt --all --check
```

The ordinary host build creates only ignored shared libraries under
`runtime/grammars`; it performs no network access. The dedicated static grammar
test compiles the same sources into its test binary:

```sh
HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1 \
  cargo +1.90.0 test --locked --offline \
    -p helix-static-grammars --features static-grammars
```

## Motor cross-build

The values below come from the selected Motor OS assembly and
`src/build-motor-os.sh`; do not replace them with machine-specific paths in a
commit:

```sh
env \
  RUSTC="$MOTOR_RUSTC" \
  RUSTDOC="$MOTOR_RUSTDOC" \
  CARGO_TARGET_X86_64_UNKNOWN_MOTOR_LINKER="$SYSROOT/bin/motor-rust-cc" \
  CC_x86_64_unknown_motor="$SYSROOT/bin/motor-clang" \
  CXX_x86_64_unknown_motor="$SYSROOT/bin/motor-clang++" \
  CXXSTDLIB_x86_64_unknown_motor="c++" \
  AR_x86_64_unknown_motor="$B/llvm-ar" \
  ARFLAGS_x86_64_unknown_motor="" \
  CARGO_TARGET_DIR="$ASSEMBLY_BUILD_ROOT/helix" \
  HELIX_DEFAULT_RUNTIME=/devtools/helix/runtime \
  HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1 \
  "$MOTOR_CARGO" build \
    --target x86_64-unknown-motor \
    --release --locked --offline --no-default-features \
    -p helix-term --bin hx
```

Stage 1 checks additionally set `DISABLED_TS_BUILD=1`. Full workspace checks
replace the last invocation with:

```sh
"$MOTOR_CARGO" check \
  --workspace --exclude xtask --target x86_64-unknown-motor \
  --locked --offline --no-default-features
```

`xtask` is host-only release tooling. It is always excluded because its
dependency on `helix-term` otherwise enables the default `git` feature.

## Image layout

Only the Motor OS development image contains Helix:

- executable: `/devtools/helix/hx`
- runtime: `/devtools/helix/runtime`
- staged runtime children: `queries`, `themes`, and `tutor`

The executable is not added to `PATH`; invoke it by its full path. Grammar
sources, Git metadata, shared libraries, build directories, configuration,
and caches are never staged into the image.

## Unsupported version-one features

Motor v1 does not support mouse input by default, suspend/job control,
external URL launch, system clipboard integration, OSC 52 clipboard access,
or LSP integration. These capabilities must fail or default cleanly. Syntax
highlighting uses the ten statically linked grammars and performs no runtime
shared-library lookup.

## Publication workflow

Agents may clone, fetch, branch, edit, test, and locally commit the Motor OS,
Helix, and tree-house repositories. Agents do not create GitHub forks, push
branches, or open pull requests; a human performs all remote publication.

During local development, the root manifest may temporarily patch
`tree-house-bindings` to `../tree-house/bindings`. That path override is not a
final dependency pin and must not appear in a published commit. After a human
pushes the tested bindings commit, replace the path with the Moturus GitHub URL
and its full 40-character revision, regenerate `Cargo.lock`, and repeat all
gates offline. The published Helix commit must contain no path dependency,
`file:` URL, absolute sibling path, branch-only dependency, or fabricated Git
revision.

After the human pushes the final Helix commit, consumers verify both its commit
and tree identities, pin the full revision, and rebuild from a fresh managed
checkout. A branch name or release tag never substitutes for a full commit
hash in build or assembly provenance.
