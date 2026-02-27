# Zed Please Extension

This is a Zed editor extension that provides Language Server Protocol (LSP) support for Please (plz) build files (e.g., BUILD, BUILD.plz). It integrates Please's `build_langserver` for features like code completion, diagnostics, go-to-definition, and more, similar to the VS Code plugin.

The extension uses a Tree-sitter grammar for Starlark (the language behind Please BUILD files) for syntax highlighting and associates the LSP server for semantic features.

## Prerequisites

- [Zed editor](https://zed.dev/) installed (latest version recommended).
- [Rust](https://www.rust-lang.org/tools/install) installed via rustup.
- Git installed.
- A terminal or command line interface.
- Access to build the Please repository - requires Go and Python (pip) for building.

## Step 1: Build the Please LSP Server (`build_langserver`)

The LSP server is part of the Please source code and must be built locally.

1. Clone the Please repository:
   ```
   git clone https://github.com/thought-machine/please.git
   cd please
   ```

2. Bootstrap Please if needed (it will install itself):
   ```
   ./bootstrap.sh
   ```

3. Build the LSP server:
   ```
   ./pleasew build //tools/build_langserver:build_langserver
   ```
   - Or if you have `plz` aliased/installed: `plz build //tools/build_langserver:build_langserver`.
   - This will output the binary in `plz-out/bin/tools/build_langserver/build_langserver`.

4. Find the absolute path to the binary:
   - Navigate to the repository root (the directory containing `.plzconfig`).
   - Run:
     ```
     echo "$(pwd)/$(plz query output //tools/build_langserver:build_langserver)"
     ```
     - Example output: `/path/to/please/repo/plz-out/bin/tools/build_langserver/build_langserver`.
   - Alternatively, use Git to ensure you're at the root:
     ```
     cd $(git rev-parse --show-toplevel)
     echo "$(pwd)/$(plz query output //tools/build_langserver:build_langserver)"
     ```
   - Make the binary executable if needed:
     ```
     chmod +x /path/to/plz-out/bin/tools/build_langserver/build_langserver
     ```
   - Test it:
     ```
     /path/to/plz-out/bin/tools/build_langserver/build_langserver --stdio
     ```
     (It should start and wait for input; Ctrl+C to exit.)

Note this path; you'll need it for the extension configuration.

## Step 2: Configure the Extension

1. Open the `.env` file in the extension directory:
   ```
   cd zed-please-extension
   ```

2. Edit the `.env` file to set the `BUILD_LANGSERVER_PATH` variable to the absolute path of the `build_langserver` binary you built in Step 1.

   Example:
   ```
   BUILD_LANGSERVER_PATH=/home/user/please/plz-out/bin/tools/build_langserver/build_langserver
   ```

**Important**: The extension requires the `BUILD_LANGSERVER_PATH` environment variable to be set in the `.env` file. Without this configuration, the extension will fail to start the language server.


## Step 3: Build the Extension

1. Install the required Rust target (WASI Preview 2):
   ```
   rustup target add wasm32-wasip2
   ```

2. Build the Wasm module:
   ```
   cargo build --release --target wasm32-wasip2
   ```
   - The output will be in `target/wasm32-wasip2/release/zed_please.wasm`.

## Step 4: Install and Enable the Extension in Zed

1. Open Zed.
2. Open the command palette (Cmd/Ctrl + Shift + P).
3. Search for and run "zed: install dev extension".
4. Select your `zed-please-extension` directory.
5. Zed will load the extension. Restart Zed if prompted.

The extension is now enabled by default for files matching the patterns (e.g., BUILD files). No further enabling is needed—Zed handles it automatically.

## Step 5: Use the Extension

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
