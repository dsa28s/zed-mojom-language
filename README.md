<!-- Copyright 2026 Dora Lee -->

# Mojom for Zed

Zed extension for Mojom IDL files.

## What This Provides

- `.mojom` and `.test-mojom` language detection
- Tree-sitter based syntax highlighting, bracket matching, indentation, outline, and text objects
- Mojom Language Server integration through `mojom-lsp`

## LSP Runtime Model

Zed extensions run their Rust code as Wasm. The extension cannot run the Rust language-server crate directly inside that Wasm module; it must return a native command for Zed to launch.

This extension resolves `mojom-lsp` in this order:

1. `lsp.mojom-lsp.binary.path` from Zed settings
2. `mojom-lsp` or `mojom-lsp.exe` from the worktree shell `PATH`
3. A downloaded release asset from `dsa28s/zed-mojom-language`

## Local LSP Development

The Mojom language server source is vendored in `lsp/mojom-lsp` so extension and server changes can ship from this repository.

For local extension development, build the server from this checkout:

```sh
cargo build --manifest-path lsp/mojom-lsp/Cargo.toml --bin mojom-lsp
```

Then point Zed at the built binary with `lsp.mojom-lsp.binary.path`, or put the binary on your `PATH`.

## Zed Settings

```json
{
  "lsp": {
    "mojom-lsp": {
      "binary": {
        "path": "/absolute/path/to/mojom-lsp"
      }
    }
  }
}
```

## Release Asset Names

To make the extension work immediately after install, publish these assets on the same GitHub repository used by `SERVER_RELEASE_REPOSITORY` in `src/lib.rs`:

- `mojom-lsp-aarch64-apple-darwin.tar.gz`
- `mojom-lsp-x86_64-apple-darwin.tar.gz`
- `mojom-lsp-aarch64-pc-windows-msvc.zip`
- `mojom-lsp-x86_64-pc-windows-msvc.zip`

Each archive should contain the binary at the archive root:

- macOS: `mojom-lsp`
- Windows: `mojom-lsp.exe`
