use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct PleaseBuildExtension;

impl zed::Extension for PleaseBuildExtension {
    fn new() -> Self {
        PleaseBuildExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // 1. Check if the user has set a custom binary path in their settings.json:
        //    "lsp": { "build_langserver": { "binary": { "path": "/custom/path/build_langserver" } } }
        let binary_path = LspSettings::for_worktree("build_langserver", worktree)
            .ok()
            .and_then(|s| s.binary)
            .and_then(|b| b.path);

        let command = match binary_path {
            Some(path) => path,
            None => {
                // 2. Fall back to searching the user's PATH
                worktree
                    .which("build_langserver")
                    .ok_or_else(|| {
                        "build_langserver not found. \
                         Build it from the Please repo (https://github.com/thought-machine/please) \
                         and either add it to your PATH, or set its location in Zed settings:\n\
                         \"lsp\": { \"build_langserver\": { \"binary\": { \"path\": \"/path/to/build_langserver\" } } }"
                            .to_string()
                    })?
            }
        };

        Ok(zed::Command {
            command,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed_extension_api::register_extension!(PleaseBuildExtension);
