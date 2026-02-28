# Please Build — Zed Extension

This is a Zed editor extension that provides Language Server Protocol (LSP) support for Please (plz) build files in the
[Zed editor](https://zed.dev/). 

It integrates Please's `build_langserver` for features like code completion, diagnostics, go-to-definition, and more, similar to the VS Code plugin.


## Prerequisites

`build_langserver` must be installed and available. Build it from the
Please source repo:

    git clone https://github.com/thought-machine/please.git
    cd please
    ./bootstrap.sh
    ./pleasew build //tools/build_langserver:build_langserver

Then either:
- Add the binary to your `$PATH`, **or**
- Point Zed to it explicitly (see Configuration below)

## Installation

Open the Extensions panel in Zed (`Cmd/Ctrl+Shift+X`), search for
**Please Build**, and click Install.

## Configuration

If `build_langserver` is not on your `$PATH`, add this to your
`~/.config/zed/settings.json`:

    "lsp": {
      "build_langserver": {
        "binary": {
          "path": "/absolute/path/to/build_langserver"
        }
      }
    }


## Use the Extension

1. Open a Please project in Zed (a directory with BUILD files).
2. Open a `BUILD` or `BUILD.plz` file.
   - Syntax highlighting should work via Tree-sitter.
   - LSP features (code completion, etc.) should activate via `build_langserver`.
3. If issues occur:
   - Check Zed's logs: Command palette > "zed: open log".
   - Ensure the `BUILD_LANGSERVER_PATH` in `.env` file is correct and the binary is executable.
   - Verify the `.env` file exists in the extension directory.
   - Rebuild and reinstall if needed.

## Troubleshooting

- If the LSP doesn't start: Verify the binary runs manually, check permissions, and ensure `BUILD_LANGSERVER_PATH` is correctly set in `.env`.
- Missing environment variable: Ensure `.env` file exists with `BUILD_LANGSERVER_PATH` set to the correct binary path.
- Update the Tree-sitter rev: Check https://github.com/tree-sitter-grammars/tree-sitter-starlark/commits/main for the latest SHA.
- For advanced customization: Refer to [Zed's extension API docs](https://zed.dev/docs/extensions).

## License

This extension is licensed under MIT as specified in Cargo.toml.

If you encounter issues or want to contribute, feel free to fork and improve!
